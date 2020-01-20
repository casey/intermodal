use std::{
  error::Error,
  fmt::{self, Display, Formatter},
  fs,
  str::FromStr,
};

use glob::glob;
use regex::Regex;

const README: &str = "README.md";

const HEADING_PATTERN: &str = "(?m)^(?P<MARKER>#+) (?P<TEXT>.*)$";

const TOC_PATTERN: &str = "(?ms)## Manual.*## General";

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
    match text.replace('\\', "").as_str() {
      "x" => Ok(Self::NotSupported),
      "+" => Ok(Self::Supported),
      "-" => Ok(Self::NotApplicable),
      "?" => Ok(Self::Unknown),
      ":x:" => Ok(Self::NotSupported),
      ":white_check_mark:" => Ok(Self::Supported),
      ":heavy_minus_sign:" => Ok(Self::NotApplicable),
      ":grey_question:" => Ok(Self::Unknown),
      _ => Err(format!("invalid status: {}", text)),
    }
  }
}

impl Display for Status {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match self {
      Self::Unknown => write!(f, ":grey_question:"),
      Self::NotApplicable => write!(f, ":heavy_minus_sign:"),
      Self::Supported => write!(f, ":white_check_mark:"),
      Self::NotSupported => write!(f, ":x:"),
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

    if number == 1000 {
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
    "| BEP                                            | Status             | {:width$} |",
    "Title",
    width = width
  ));

  lines.push(format!(
    "|:----------------------------------------------:|:------------------:|:{:-<width$}-|",
    "",
    width = width
  ));

  for (bep, original) in beps.into_iter().zip(originals) {
    assert_eq!(bep.number, original.number);
    lines.push(format!(
      "| [{:02}](http://bittorrent.org/beps/bep_{:04}.html) | {:18} | {:width$} |",
      bep.number,
      bep.number,
      original.status.to_string(),
      bep.title,
      width = width
    ));
  }

  let table = lines.join("\n");

  let readme = &[before.trim(), "", &table, "", after.trim()].join("\n");

  let header_re = Regex::new(HEADING_PATTERN)?;

  let mut toc = Vec::new();
  for captures in header_re.captures_iter(&readme).skip(2) {
    let marker = captures.name("MARKER").unwrap().as_str();
    let text = captures.name("TEXT").unwrap().as_str();
    let level = marker.len();
    let indentation = " ".repeat((level - 2) * 2);
    let slug = text.to_lowercase().replace(' ', "-");
    toc.push(format!("{}- [{}](#{})", indentation, text, slug));
  }

  let toc = toc.join("\n");

  let toc_re = Regex::new(TOC_PATTERN)?;

  let readme = toc_re.replace(
    readme,
    format!("## Manual\n\n{}\n\n## General", toc).as_str(),
  );

  fs::write(README, readme.as_bytes())?;

  Ok(())
}
