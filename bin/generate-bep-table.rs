use std::{collections::BTreeMap, fs};

use glob::glob;
use regex::Regex;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
  let title_re = Regex::new("(?m)^:Title: (?P<title>.*)$")?;

  let mut beps = BTreeMap::new();

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

    beps.insert(number, title);
  }

  let width = beps.iter().map(|(_, title)| title.len()).max().unwrap_or(0);

  println!("| BEP  | Status | {:width$} |", "Title", width = width);

  println!("|------|:------:|:{:-<width$}-|", "", width = width);
  for (number, title) in beps {
    println!(
      "| [{:04}](http://bittorrent.org/beps/bep_{:04}.html)  |   ??   | {:width$} |",
      number,
      number,
      title,
      width = width
    );
  }

  Ok(())
}
