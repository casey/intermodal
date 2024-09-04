use {
  regex::{Captures, Regex},
  std::{fs, process::Command, str},
};

fn author(pr: u64) -> String {
  eprintln!("#{pr}");
  let output = Command::new("sh")
    .args([
      "-c",
      &format!("gh pr view {pr} --json author | jq -r .author.login"),
    ])
    .output()
    .unwrap();

  assert!(
    output.status.success(),
    "{}",
    String::from_utf8_lossy(&output.stderr)
  );

  str::from_utf8(&output.stdout).unwrap().trim().to_owned()
}

fn main() {
  fs::write(
    "CHANGELOG.md",
    &*Regex::new(r"\(#(\d+)( by @[a-z]+)?\)")
      .unwrap()
      .replace_all(
        &fs::read_to_string("CHANGELOG.md").unwrap(),
        |captures: &Captures| {
          let pr = captures[1].parse().unwrap();
          let contributor = author(pr);
          format!("([#{pr}](https://github.com/casey/intermodal/pull/{pr}) by [{contributor}](https://github.com/{contributor}))")
        },
      ),
  )
  .unwrap();
}
