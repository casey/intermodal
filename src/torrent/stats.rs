use crate::common::*;

#[derive(StructOpt)]
pub(crate) struct Stats {
  #[structopt(long = "limit", short = "l")]
  limit: Option<u64>,
  #[structopt(long = "extract-pattern", short = "e")]
  extract_patterns: Vec<Regex>,
  #[structopt(name = "INPUT", long = "input", short = "i")]
  input: PathBuf,
}

impl Stats {
  pub(crate) fn run(self, env: &Environment, unstable: bool) -> Result<(), Error> {
    if !unstable {
      return Err(Error::Unstable {
        feature: "torrent stats subcommand",
      });
    }

    let path = env.resolve(self.input);

    let mut extractor = Extractor::new(&self.extract_patterns);

    for result in WalkDir::new(path).sort_by(|a, b| a.file_name().cmp(b.file_name())) {
      if extractor.torrents >= self.limit.unwrap_or(u64::MAX) {
        break;
      }

      let entry = result?;

      extractor.process(entry.path());
    }

    println!("Torrents processed: {}", extractor.torrents);
    println!("Read failed:        {}", extractor.io_errors);
    println!("Decode failed:      {}", extractor.bencode_decode_errors);

    let mut paths = extractor.paths.into_iter().collect::<Vec<(String, u64)>>();
    paths.sort_by_key(|(_, count)| Reverse(*count));
    let max = paths.iter().map(|(_, count)| *count).max().unwrap_or(0);
    let width = max.to_string().len();

    println!("Keys:");
    for (key, count) in &paths {
      if key.starts_with("info/files") {
        continue;
      }
      println!("{:<width$} - {}", count, key, width = width);
    }
    for (key, count) in paths {
      if key.starts_with("info/files") {
        println!("{:<width$} - {}", count, key, width = width);
      }
    }

    if !extractor.values.is_empty() {
      let values = extractor
        .values
        .into_iter()
        .collect::<Vec<(String, Vec<String>)>>();

      println!("Values:");
      for (pattern, values) in values {
        println!("{}: ", pattern);
        for (i, value) in values.iter().enumerate() {
          if i > 0 {
            print!(", ");
          }
          print!("{}", value);
        }
      }
    }

    Ok(())
  }
}

struct Extractor {
  bencode_decode_errors: u64,
  io_errors: u64,
  paths: HashMap<String, u64>,
  regex_set: RegexSet,
  torrents: u64,
  values: HashMap<String, Vec<String>>,
  current_path: String,
}

impl Extractor {
  fn new(regexes: &[Regex]) -> Extractor {
    let regex_set = RegexSet::new(regexes.iter().map(Regex::as_str)).unwrap();

    Extractor {
      bencode_decode_errors: 0,
      io_errors: 0,
      paths: HashMap::new(),
      torrents: 0,
      values: HashMap::new(),
      current_path: String::new(),
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

    let contents = match fs::read(&path) {
      Ok(contents) => contents,
      Err(_) => {
        self.io_errors += 1;
        return;
      }
    };

    let value = match bencode::Value::decode(&contents) {
      Ok(value) => value,
      Err(_) => {
        self.bencode_decode_errors += 1;
        return;
      }
    };

    self.extract(&value);
  }

  fn extract(&mut self, value: &bencode::Value) {
    use bencode::Value::*;

    let matches = self.regex_set.matches(&self.current_path);

    for i in matches.iter() {
      let pattern = &self.regex_set.patterns()[i];
      if let Some(values) = self.values.get_mut(pattern) {
        values.push(value.to_string());
      } else {
        self.values.insert(pattern.clone(), vec![value.to_string()]);
      }
    }

    let starting_length = self.current_path.len();

    if let Dict(items) = value {
      for (key, value) in items {
        match String::from_utf8_lossy(key) {
          Cow::Borrowed(s) => self.current_path.push_str(s),
          Cow::Owned(s) => self.current_path.push_str(&s),
        }
        self.paths.increment_ref(&self.current_path);
        self.current_path.push('/');
        self.extract(value);
        self.current_path.truncate(starting_length);
      }
    } else if let List(values) = value {
      if self.current_path.pop().is_some() {
        self.current_path.push('*');
      }
      for value in values {
        self.extract(value);
      }
      self.current_path.truncate(starting_length);
    }
  }
}
