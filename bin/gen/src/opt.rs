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
fn blank(path: impl AsRef<Path>, title: &str) {
  let path = path.as_ref();

  info!("Writing blank page to `{}`…", path.display());

  let text = format!(
    "
  # {}

  This page intentionally left blank.
  ",
    title
  );

  fs::write(path, text)?;
}

#[throws]
fn clean_dir(dir: impl AsRef<Path>) {
  let dir = dir.as_ref();

  info!("Cleaning `{}`…", dir.display());

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
    info!("Generating changelog…");
    let changelog = Changelog::new(&project)?;

    let dst = project.root.join("CHANGELOG.md");

    fs::write(dst, changelog.to_string())?;
  }

  #[throws]
  pub(crate) fn completion_scripts(project: &Project) {
    info!("Generating completion scripts…");
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
    info!("Generating readme…");
    let template = project.root.join("bin/gen/templates/README.md");

    let readme = Readme::load(&project.config, &template)?;

    let text = readme.render_newline()?;

    fs::write(project.root.join("README.md"), text)?;
  }

  #[throws]
  pub(crate) fn book(project: &Project) {
    info!("Generating book…");

    blank(project.root.join("book/src/commands.md"), "Commands")?;
    blank(project.root.join("book/src/bittorrent.md"), "BitTorrent")?;
    blank(project.root.join("book/src/references.md"), "References")?;

    let commands = project.root.join("book/src/commands/");

    clean_dir(&commands)?;

    for subcommand in &project.bin.subcommands {
      let page = subcommand.page()?;

      let dst = commands.join(format!("{}.md", subcommand.slug()));

      fs::write(dst, page)?;
    }

    let references = project.root.join("book/src/references/");
    clean_dir(&references)?;

    for section in &project.config.references {
      let text = section.render_newline()?;

      let path = project.root.join("book/src").join(section.path());

      fs::write(path, text)?;
    }

    let summary = Summary::new(project);

    let text = summary.render_newline()?;

    fs::write(project.root.join("book/src/SUMMARY.md"), text)?;

    let introduction = Introduction::new(&project.config);

    let text = introduction.render_newline()?;

    fs::write(project.root.join("book/src/introduction.md"), text)?;
  }

  #[throws]
  pub(crate) fn man(project: &Project) {
    info!("Generating man pages…");
    let mans = project.root.join("man");

    clean_dir(&mans)?;

    for subcommand in &project.bin.subcommands {
      let man = subcommand.man()?;

      let dst = mans.join(format!("{}.1", subcommand.slug()));

      info!("Writing man page to `{}`", dst.display());

      fs::write(dst, man)?;
    }
  }
}
