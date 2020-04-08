use crate::common::*;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub(crate) struct Subcommand {
  pub(crate) bin: String,
  pub(crate) command: Vec<String>,
  pub(crate) help: String,
  pub(crate) man: String,
  pub(crate) subcommands: Vec<String>,
}

trait CommandExt {
  #[throws]
  fn out(&mut self) -> String;
}

impl CommandExt for Command {
  #[throws]
  fn out(&mut self) -> String {
    let output = self.stdout(Stdio::piped()).output()?;

    if !output.status.success() {
      throw!(anyhow!("Command `{:?}` failed: {}", self, output.status));
    }

    let text = String::from_utf8(output.stdout)?;

    text
  }
}

impl Subcommand {
  #[throws]
  pub(crate) fn new(bin: &str, command: Vec<String>) -> Self {
    let help = Command::new(bin)
      .args(command.as_slice())
      .arg("--help")
      .out()?;

    const MARKER: &str = "\nSUBCOMMANDS:\n";

    let mut subcommands = Vec::new();

    if let Some(marker) = help.find(MARKER) {
      let block = &help[marker + MARKER.len()..];

      for line in block.lines() {
        let name = line.trim().split_whitespace().next().unwrap();
        subcommands.push(name.into());
      }
    }

    let command_line = format!("{} {}", bin, command.join(" "));

    let name = command_line
      .split('/')
      .last()
      .unwrap()
      .trim()
      .replace(" ", "\\ ");

    let description = if command.is_empty() {
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

    let tmp = tempfile::tempdir()?;

    fs::write(tmp.path().join("include"), include)?;

    let include = tmp.path().join("include").to_string_lossy().into_owned();

    let version = Command::new(bin)
      .arg("--version")
      .out()?
      .split_whitespace()
      .nth(1)
      .unwrap()
      .to_owned();

    let output = Command::new("help2man")
      .args(&[
        "--include",
        &include,
        "--manual",
        "Intermodal Manual",
        "--no-info",
        "--source",
        &format!("Intermodal {}", version),
      ])
      .arg(&command_line)
      .stdout(Stdio::piped())
      .output()?;

    if !output.status.success() {
      throw!(anyhow!(
        "Failed to generate man page for  `{}` failed: {}",
        command_line,
        output.status
      ));
    }

    let man = str::from_utf8(&output.stdout)?
      .replace("📦 ", "\n")
      .replace("\n.SS ", "\n.SH ")
      .replace("\"USAGE:\"", "\"SYNOPSIS:\"");

    let re = Regex::new(r"(?ms).SH DESCRIPTION.*?.SH").unwrap();

    let man = re.replace(&man, ".SH").into_owned();

    Self {
      bin: bin.into(),
      command,
      help,
      man,
      subcommands,
    }
  }

  pub(crate) fn slug(&self) -> String {
    let mut slug = self.bin.split('/').last().unwrap().to_owned();

    for name in &self.command {
      slug.push('-');
      slug.push_str(&name);
    }

    slug
  }

  pub(crate) fn command_line(&self) -> String {
    self.slug().replace('-', " ")
  }

  pub(crate) fn page(&self) -> String {
    format!("# `{}`\n```\n{}\n```", self.command_line(), self.help)
  }
}
