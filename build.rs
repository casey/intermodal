use std::{
  io::{self, Error, ErrorKind},
  process::Command,
};

fn is_inside_git_work_tree() -> bool {
  Command::new("git")
    .arg("rev-parse")
    .arg("--is-inside-work-tree")
    .output()
    .map(|output| String::from_utf8_lossy(&output.stdout).trim() == "true")
    .unwrap_or(false)
}

fn error(message: String) -> io::Error {
  Error::new(ErrorKind::Other, message)
}

fn commit_hash() -> io::Result<String> {
  let output = Command::new("git").arg("rev-parse").arg("HEAD").output()?;

  if !output.status.success() {
    return Err(error(format!(
      "Status error: `git rev-parse HEAD` failed: {}",
      output.status
    )));
  }

  let hash = String::from_utf8_lossy(&output.stdout)
    .into_owned()
    .trim()
    .to_owned();

  if !hash.chars().all(|c| "0123456789abcdef".contains(c)) {
    return Err(error(format!(
      "Invalid hash from `git rev-parse HEAD`: {}",
      hash
    )));
  }

  Ok(hash)
}

fn main() -> io::Result<()> {
  if is_inside_git_work_tree() {
    let hash = commit_hash()?;
    println!("cargo:rustc-env=GIT_HEAD_PARTIAL_HASH= ({})", &hash[0..12]);
  } else {
    println!("cargo:rustc-env=GIT_HEAD_PARTIAL_HASH=");
  };

  Ok(())
}
