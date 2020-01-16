use std::{
  error::Error,
  fmt::{self, Display, Formatter},
  fs,
  str::FromStr,
};

use glob::glob;
use regex::Regex;

const README: &str = "README.md";

struct Bep {
  number: usize,
  title: String,
  status: Status,
}

enum Status {
  Unknown,
  NotApplicable,
  Supported,
  NotSupported,
}

impl FromStr for Status {
  type Err = String;

  fn from_str(text: &str) -> Result<Self, Self::Err> {
    match text {
      "?" => Ok(Self::Unknown),
      "N/A" => Ok(Self::NotApplicable),
      "-" => Ok(Self::NotSupported),
      "✗" => Ok(Self::NotSupported),
      "+" => Ok(Self::Supported),
      "✓" => Ok(Self::Supported),
      _ => Err(format!("invalid status: {}", text)),
    }
  }
}

impl Display for Status {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match self {
      Self::Unknown => write!(f, "?"),
      Self::NotApplicable => write!(f, "N/A"),
      Self::Supported => write!(f, "✓"),
      Self::NotSupported => write!(f, "✗"),
    }
  }
}

fn main() -> Result<(), Box<dyn Error>> {
  let title_re = Regex::new("(?m)^:Title: (?P<title>.*)$")?;

  let mut beps = Vec::new();

  for result in glob("tmp/bittorrent.org/beps/bep_*.rst")? {
    let path = result?;

    let number = path
      .file_stem()
      .unwrap()
      .to_string_lossy()
      .split('_')
      .nth(1)
      .unwrap()
      .parse::<usize>()?;

    if number == 1000 || number == 0 || number == 1 || number == 2 {
      continue;
    }

    let rst = fs::read_to_string(path)?;

    let title = title_re
      .captures(&rst)
      .unwrap()
      .name("title")
      .unwrap()
      .as_str()
      .trim()
      .to_owned();

    beps.push(Bep {
      status: Status::Unknown,
      number,
      title,
    });
  }

  beps.sort_by_key(|bep| bep.number);

  let table_re = Regex::new(
    r"(?mx)
    ^[|]\ BEP.*
    (
      \n
      [|]
      .*
    )*
  ",
  )?;

  let readme = fs::read_to_string(README)?;

  let parts = table_re.split(&readme).into_iter().collect::<Vec<&str>>();

  assert_eq!(parts.len(), 2);

  let before = parts[0];
  let after = parts[1];
  let original = table_re
    .captures(&readme)
    .unwrap()
    .get(0)
    .unwrap()
    .as_str()
    .trim();

  let row_re = Regex::new(
    r"(?x)
    ^
    \|
    \s*
    \[
    (?P<number>[0-9]+)
    \]
    .*
    \s*
    \|
    (?P<status>.*)
    \|
    (?P<title>.*)
    \|
    $
  ",
  )?;

  let mut originals = Vec::new();

  for row in original.lines().skip(2) {
    let captures = row_re.captures(row).unwrap();
    originals.push(Bep {
      number: captures.name("number").unwrap().as_str().parse()?,
      status: captures.name("status").unwrap().as_str().trim().parse()?,
      title: captures.name("title").unwrap().as_str().to_owned(),
    });
  }

  assert_eq!(originals.len(), beps.len());

  let mut lines = Vec::new();

  let width = beps.iter().map(|bep| bep.title.len()).max().unwrap_or(0);

  lines.push(format!(
    "| BEP                                            | Status | {:width$} |",
    "Title",
    width = width
  ));

  lines.push(format!(
    "|:----------------------------------------------:|:------:|:{:-<width$}-|",
    "",
    width = width
  ));

  for (bep, original) in beps.into_iter().zip(originals) {
    assert_eq!(bep.number, original.number);
    lines.push(format!(
      "| [{:02}](http://bittorrent.org/beps/bep_{:04}.html) |   {:3}    | {:width$} |",
      bep.number,
      bep.number,
      original.status,
      bep.title,
      width = width
    ));
  }

  let table = lines.join("\n");

  let readme = &[before.trim(), "", &table, "", after.trim(), ""].join("\n");

  fs::write(README, readme)?;

  Ok(())
}
