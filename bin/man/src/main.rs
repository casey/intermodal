use crate::common::*;

mod bin;
mod common;
mod subcommand;

#[throws]
fn clean(dir: impl AsRef<Path>) {
  let dir = dir.as_ref();
  fs::remove_dir_all(dir)?;
  fs::create_dir_all(dir)?;
}

#[throws]
fn write(dst: impl AsRef<Path>, contents: &str) {
  let dst = dst.as_ref();
  println!("Writing `{}`…", dst.display());
  fs::write(dst, contents)?;
}

#[throws]
fn main() {
  let bin = Bin::new("target/debug/imdl")?;

  clean("man")?;
  clean("book/src/commands")?;

  let mut pages = "- [Commands](./commands.md)\n".to_string();

  for subcommand in bin.subcommands {
    let slug = subcommand.slug();

    let dst = format!("man/{}.1", slug);
    write(dst, &subcommand.man)?;

    let dst = format!("book/src/commands/{}.md", slug);
    write(dst, &subcommand.page())?;

    pages.push_str(&format!(
      "  - [`{}`](./commands/{}.md)\n",
      subcommand.command_line(),
      slug
    ))
  }

  pages.push('\n');

  let path = "book/src/SUMMARY.md";

  let original = fs::read_to_string(path)?;

  let re = Regex::new(r"(?ms)^- \[Commands\]\(./commands.md\).*?\n\n").unwrap();

  let text = re.replace(&original, pages.as_str()).into_owned();

  fs::write(path, text)?;
}
