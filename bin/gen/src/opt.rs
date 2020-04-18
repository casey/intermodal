use crate::common::*;

#[derive(StructOpt)]
pub(crate) enum Opt {
  #[structopt(about("Update all generated docs"))]
  All,
  #[structopt(about("Generate the changelog"))]
  Changelog,
  #[structopt(about("Print a commit template to standard output"))]
  CommitTemplate,
  #[structopt(about("Print possible values for `type` field of commit metadata"))]
  CommitTypes,
  #[structopt(about("Generate completion scripts"))]
  CompletionScripts,
  #[structopt(about("Generate readme"))]
  Readme,
  #[structopt(about("Generate book"))]
  Book,
  #[structopt(about("Generate man pages"))]
  Man,
}

#[throws]
fn clean_dir(dir: impl AsRef<Path>) {
  let dir = dir.as_ref();

  eprintln!("Cleaning `{}`…", dir.display());

  if dir.is_dir() {
    fs::remove_dir_all(dir)?;
  }

  fs::create_dir_all(dir)?;
}

impl Opt {
  #[throws]
  pub(crate) fn run(self, project: &Project) {
    match self {
      Self::Changelog => Self::changelog(project)?,
      Self::CommitTemplate => {
        println!("{}", Metadata::default().to_string());
      }
      Self::CommitTypes => {
        for kind in Kind::VARIANTS {
          println!("{}", kind)
        }
      }
      Self::CompletionScripts => Self::completion_scripts(project)?,
      Self::Readme => Self::readme(project)?,
      Self::Book => Self::book(project)?,
      Self::Man => Self::man(project)?,
      Self::All => {
        Self::changelog(project)?;
        Self::completion_scripts(project)?;
        Self::readme(project)?;
        Self::book(project)?;
        Self::man(project)?;
      }
    }
  }

  #[throws]
  pub(crate) fn changelog(project: &Project) {
    eprintln!("Generating changelog…");
    let changelog = Changelog::new(&project)?;

    let dst = project.root.join("CHANGELOG.md");

    fs::write(dst, changelog.to_string())?;
  }

  #[throws]
  pub(crate) fn completion_scripts(project: &Project) {
    eprintln!("Generating completion scripts…");
    let completions = project.root.join("completions");

    clean_dir(&completions)?;

    cmd!(
      "cargo",
      "run",
      "--package",
      "imdl",
      "completions",
      "--dir",
      completions
    )
    .status()?
    .into_result()?;
  }

  #[throws]
  pub(crate) fn readme(project: &Project) {
    eprintln!("Generating readme…");
    let template = project.root.join("bin/gen/templates/README.md");

    let readme = Readme::load(&project.config, &template)?;

    let mut text = readme.render()?;
    text.push('\n');

    fs::write(project.root.join("README.md"), text)?;
  }

  #[throws]
  pub(crate) fn book(project: &Project) {
    eprintln!("Generating book…");
    let commands = project.root.join("book/src/commands/");

    clean_dir(&commands)?;

    for subcommand in &project.bin.subcommands {
      let page = subcommand.page()?;

      let dst = commands.join(format!("{}.md", subcommand.slug()));

      fs::write(dst, page)?;
    }

    let summary = Summary::new(&project.bin);

    let mut text = summary.render()?;
    text.push('\n');

    fs::write(project.root.join("book/src/SUMMARY.md"), text)?;

    let introduction = Introduction::new(&project.config);

    let mut text = introduction.render()?;
    text.push('\n');

    fs::write(project.root.join("book/src/introduction.md"), text)?;
  }

  #[throws]
  pub(crate) fn man(project: &Project) {
    eprintln!("Generating man pages…");
    let mans = project.root.join("man");

    clean_dir(&mans)?;

    for subcommand in &project.bin.subcommands {
      let man = subcommand.man()?;

      let dst = mans.join(format!("{}.1", subcommand.slug()));

      eprintln!("Writing man page to `{}`", dst.display());

      fs::write(dst, man)?;
    }
  }
}
