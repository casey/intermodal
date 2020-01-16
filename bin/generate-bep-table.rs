use std::fs;

use glob::glob;
use regex::Regex;

const README: &str = "README.md";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

struct Bep {
  number: usize,
  title: String,
  status: Option<String>,
}

fn main() -> Result<()> {
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
      status: None,
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
      status: Some(captures.name("status").unwrap().as_str().to_owned()),
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
      "| [{:02}](http://bittorrent.org/beps/bep_{:04}.html) |   {}   | {:width$} |",
      bep.number,
      bep.number,
      original.status.unwrap().trim(),
      bep.title,
      width = width
    ));
  }

  let table = lines.join("\n");

  let readme = &[before.trim(), "", &table, "", after.trim(), ""].join("\n");

  fs::write(README, readme)?;

  Ok(())
}
