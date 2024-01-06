use std::{
  error::Error,
  io::{stdout, Write},
  path::{Path, PathBuf},
  process::Command,
  thread::sleep,
  time::Duration,
};

type Result<T, E = Box<dyn Error>> = std::result::Result<T, E>;

const SCRIPT: &str = "
  ls -l 9front
  imdl torrent create --input 9front
  imdl torrent show --input 9front.torrent
  imdl torrent verify --input 9front.torrent
  imdl torrent link --input 9front.torrent
";

const PROMPT: &str = "\x1b[0;34m$\x1b[0m ";

const CPM: u64 = 1000;

fn commands() -> Vec<Vec<&'static str>> {
  SCRIPT
    .lines()
    .map(|line| line.trim())
    .filter(|line| !line.is_empty())
    .map(|line| line.split(' ').collect())
    .collect()
}

fn print(text: &str) -> Result<()> {
  stdout().write_all(text.as_bytes())?;
  stdout().flush()?;
  Ok(())
}

fn replace(bin: &str) -> Result<PathBuf> {
  match bin {
    "ls" => Ok("exa".into()),
    "imdl" => Ok(Path::new("./target/release/imdl").canonicalize()?),
    _ => Ok(bin.into()),
  }
}

fn run(command: &[&str]) -> Result<()> {
  Command::new(replace(command[0])?)
    .args(&command[1..])
    .current_dir("tmp")
    .status()?;
  Ok(())
}

fn main() -> Result<()> {
  let char_delay = Duration::from_millis(1000 * 60 / CPM);
  let line_delay = char_delay * 7;
  let enter_delay = char_delay * 5;

  for (i, command) in commands().iter().enumerate() {
    print(PROMPT)?;

    if i > 0 {
      sleep(line_delay);
    }

    let line = command.join(" ");

    for c in line.chars() {
      sleep(char_delay);
      print(&c.to_string())?;
    }

    sleep(enter_delay);
    print("\n")?;

    run(command)?;
  }

  Ok(())
}
