use crate::common::*;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub(crate) struct BinSubcommand {
  pub(crate) bin: PathBuf,
  pub(crate) command: Vec<String>,
  pub(crate) subcommands: Vec<String>,
}

impl BinSubcommand {
  #[throws]
  pub(crate) fn new(bin: &Path, command: Vec<String>) -> Self {
    let wide_help = Command::new(bin)
      .args(command.as_slice())
      .env("IMDL_TERM_WIDTH", "200")
      .arg("--help")
      .out()?;

    const MARKER: &str = "\nSUBCOMMANDS:\n";

    let mut subcommands = Vec::new();

    if let Some(marker) = wide_help.find(MARKER) {
      let block = &wide_help[marker + MARKER.len()..];

      for line in block.lines() {
        let name = line.split_whitespace().next().unwrap();
        subcommands.push(name.into());
      }
    }

    Self {
      bin: bin.into(),
      command,
      subcommands,
    }
  }

  #[throws]
  fn help(&self) -> String {
    info!("Getting help for `{}`", self.command_line());

    Command::new(&self.bin)
      .args(self.command.as_slice())
      .env("IMDL_TERM_WIDTH", "80")
      .arg("--help")
      .out()?
  }

  #[throws]
  pub(crate) fn man(&self) -> String {
    let command_line = self.command_line();

    info!("Generating man page for `{}`", command_line);

    let name = command_line.replace(' ', "\\ ");

    let help = self.help()?;

    let description = if self.command.is_empty() {
      "A 40' shipping container for the Internet".to_string()
    } else {
      help.lines().nth(1).unwrap().into()
    };

    let include = format!(
      "\
[NAME]
\\fB{}\\fR
- {}
",
      name, description
    );

    let tmp = tempfile::tempdir().context(error::Tempdir)?;

    let include_path = tmp.path().join("include");

    fs::write(&include_path, include).context(error::Filesystem {
      path: &include_path,
    })?;

    let version = cmd!(&self.bin, "--version")
      .out()?
      .split_whitespace()
      .nth(1)
      .unwrap()
      .to_owned();

    info!("Running help2man for `{}`", command_line);

    let mut command = self.bin.as_os_str().to_owned();
    for arg in &self.command {
      command.push(" ");
      command.push(arg);
    }

    let output = cmd!(
      "help2man",
      "--include",
      &include_path,
      "--manual",
      "Intermodal Manual",
      "--no-info",
      "--source",
      &format!("Intermodal {}", version),
      command
    )
    .out()?;

    let man = output
      .replace("📦 ", "\n")
      .replace("\n.SS ", "\n.SH ")
      .replace("\"USAGE:\"", "\"SYNOPSIS:\"");

    let re = Regex::new(r"(?ms).SH DESCRIPTION.*?.SH").unwrap();

    let man = re.replace(&man, ".SH").into_owned();

    man
  }

  pub(crate) fn slug(&self) -> String {
    let mut slug = "imdl".to_string();

    for name in &self.command {
      slug.push('-');
      slug.push_str(name);
    }

    slug
  }

  pub(crate) fn command_line(&self) -> String {
    let mut line = "imdl".to_string();

    for name in &self.command {
      line.push(' ');
      line.push_str(name);
    }

    line
  }

  #[throws]
  pub(crate) fn page(&self) -> String {
    let help = self.help()?;
    format!("# `{}`\n```\n{}\n```", self.command_line(), help)
  }
}
