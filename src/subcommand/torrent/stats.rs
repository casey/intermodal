use crate::common::*;

#[derive(StructOpt)]
#[structopt(
  help_message(consts::HELP_MESSAGE),
  version_message(consts::VERSION_MESSAGE),
  about("Show statistics about a collection of .torrent files.")
)]
pub(crate) struct Stats {
  #[structopt(
    long = "limit",
    short = "l",
    value_name = "N",
    help = "Stop after processing `N` torrents. Useful when processing large collections of \
            `.torrent` files."
  )]
  limit: Option<u64>,
  #[structopt(
    long = "extract-pattern",
    short = "e",
    value_name = "REGEX",
    empty_values(false),
    help = "Extract and display values under key paths that match `REGEX`. Subkeys of a \
            bencodeded dictionary are delimited by `/`, and values of a bencoded list are \
            delmited by `*`. For example, given the following bencoded dictionary `{\"foo\": \
            [{\"bar\": {\"baz\": 2}}]}`, the value `2`'s key path will be `foo*bar/baz`. The \
            value `2` would be displayed if any of `bar`, `foo[*]bar/baz`, or `foo.*baz` were \
            passed to `--extract-pattern."
  )]
  extract_patterns: Vec<Regex>,
  #[structopt(
    long = "input",
    short = "i",
    value_name = "PATH",
    empty_values(false),
    parse(from_os_str),
    help = "Search `PATH` for torrents. May be a directory or a single torrent file."
  )]
  input: PathBuf,
  #[structopt(
    long = "print",
    short = "p",
    help = "Pretty print the contents of each torrent as it is processed."
  )]
  print: bool,
}

impl Stats {
  pub(crate) fn run(self, env: &mut Env, options: &Options) -> Result<(), Error> {
    options.require_unstable("torrent stats subcommand")?;

    let path = env.resolve(self.input)?;

    let mut extractor = Extractor::new(self.print, &self.extract_patterns);

    for result in WalkBuilder::new(path)
      .standard_filters(false)
      .hidden(true)
      .sort_by_file_name(Ord::cmp)
      .build()
    {
      if extractor.torrents >= self.limit.unwrap_or(u64::MAX) {
        break;
      }

      let entry = result?;

      extractor.process(entry.path());
    }

    errln!(env, "Torrents processed: {}", extractor.torrents)?;
    errln!(env, "Read failed:        {}", extractor.io_errors)?;
    errln!(
      env,
      "Decode failed:      {}",
      extractor.bencode_decode_errors
    )?;

    let mut paths = extractor.paths.into_iter().collect::<Vec<(String, u64)>>();
    paths.sort_by_key(|(_, count)| Reverse(*count));
    let max = paths.iter().map(|(_, count)| *count).max().unwrap_or(0);
    let width = max.to_string().len();

    if !paths.is_empty() {
      errln!(env, "Keys:")?;
      for (key, count) in &paths {
        if key.starts_with("info/files") {
          continue;
        }
        errln!(env, "{:<width$} - {}", count, key, width = width)?;
      }
      for (key, count) in paths {
        if key.starts_with("info/files") {
          errln!(env, "{:<width$} - {}", count, key, width = width)?;
        }
      }
    }

    if !extractor.values.is_empty() {
      let values = extractor
        .values
        .into_iter()
        .collect::<Vec<(String, Vec<String>)>>();

      errln!(env, "Values:")?;
      for (pattern, values) in values {
        err!(env, "{}: ", pattern)?;
        for (i, value) in values.iter().enumerate() {
          if i > 0 {
            err!(env, ", ")?;
          }
          err!(env, "{}", value)?;
        }
        errln!(env)?;
      }
    }

    Ok(())
  }
}

struct Extractor {
  bencode_decode_errors: u64,
  current_path: String,
  io_errors: u64,
  paths: HashMap<String, u64>,
  print: bool,
  regex_set: RegexSet,
  torrents: u64,
  values: HashMap<String, Vec<String>>,
}

impl Extractor {
  fn new(print: bool, regexes: &[Regex]) -> Self {
    let regex_set = RegexSet::new(regexes.iter().map(Regex::as_str))
      .invariant_unwrap("Regexes already validated by compilation");

    Self {
      bencode_decode_errors: 0,
      io_errors: 0,
      paths: HashMap::new(),
      torrents: 0,
      values: HashMap::new(),
      current_path: String::new(),
      print,
      regex_set,
    }
  }

  fn process(&mut self, path: &Path) {
    if !path.is_file() || path.extension() != Some(OsStr::new("torrent")) {
      return;
    }

    if self.torrents % 10000 == 0 {
      eprintln!("Processing torrent {}...", self.torrents);
    }

    self.torrents += 1;

    let Ok(contents) = fs::read(path) else {
      self.io_errors += 1;
      return;
    };

    if let Ok(value) = bendy::serde::de::from_bytes::<Value>(&contents) {
      self.extract(&value);
      if self.print {
        eprintln!("{}:\n{}", path.display(), Self::pretty_print(&value));
      }
    } else {
      self.bencode_decode_errors += 1;
    }
  }

  fn extract(&mut self, value: &Value) {
    let matches = self.regex_set.matches(&self.current_path);

    for i in matches {
      let pattern = &self.regex_set.patterns()[i];
      if let Some(values) = self.values.get_mut(pattern) {
        values.push(Self::pretty_print(value));
      } else {
        self
          .values
          .insert(pattern.clone(), vec![Self::pretty_print(value)]);
      }
    }

    let starting_length = self.current_path.len();

    match value {
      Value::List(list) => {
        if self.current_path.pop().is_some() {
          self.current_path.push('*');
        }
        for value in list {
          self.extract(value);
        }
        self.current_path.truncate(starting_length);
      }
      Value::Dict(dict) => {
        for (key, value) in dict {
          match String::from_utf8_lossy(key) {
            Cow::Borrowed(s) => self.current_path.push_str(s),
            Cow::Owned(s) => self.current_path.push_str(&s),
          }
          self.paths.increment_ref(&self.current_path);
          self.current_path.push('/');
          self.extract(value);
          self.current_path.truncate(starting_length);
        }
      }
      Value::Integer(_) | Value::Bytes(_) => {}
    }
  }

  fn pretty_print(value: &Value) -> String {
    let mut buffer = String::new();
    Self::pretty_print_inner(value, &mut buffer);
    buffer
  }

  fn pretty_print_inner(value: &Value, buffer: &mut String) {
    match value {
      Value::List(list) => {
        buffer.push('[');
        for (i, value) in list.iter().enumerate() {
          if i > 0 {
            buffer.push_str(", ");
          }
          Self::pretty_print_inner(value, buffer);
        }
        buffer.push(']');
      }
      Value::Dict(dict) => {
        buffer.push('{');
        for (i, (key, value)) in dict.iter().enumerate() {
          if i > 0 {
            buffer.push_str(", ");
          }
          Self::pretty_print_string(key, buffer);
          buffer.push_str(": ");
          Self::pretty_print_inner(value, buffer);
        }
        buffer.push('}');
      }
      Value::Integer(integer) => buffer.push_str(&integer.to_string()),
      Value::Bytes(bytes) => {
        Self::pretty_print_string(bytes, buffer);
      }
    }
  }

  fn pretty_print_string(string: &[u8], buffer: &mut String) {
    if let Ok(text) = str::from_utf8(string) {
      buffer.push('\"');
      buffer.push_str(text);
      buffer.push('\"');
    } else {
      buffer.push('<');
      for byte in string {
        buffer.push_str(&format!("{byte:02X}"));
      }
      buffer.push('>');
    }
  }
}
