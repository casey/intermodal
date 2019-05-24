use std::{error::Error, process::Command, str};

fn main() -> Result<(), Box<dyn Error>> {
  let stdout = Command::new("git")
    .arg("rev-parse")
    .arg("HEAD")
    .output()?
    .stdout;
  let hash = str::from_utf8(&stdout)?;
  println!("cargo:rustc-env=GIT_HEAD_PARTIAL_HASH={}", &hash[0..12]);
  Ok(())
}
