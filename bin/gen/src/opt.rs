use crate::common::*;

#[derive(StructOpt)]
pub(crate) enum Opt {
  #[structopt(about("Update all generated docs"))]
  All,
  #[structopt(about("Generate book"))]
  Book,
  #[structopt(about("Generate the changelog"))]
  Changelog,
  #[structopt(about("Print a commit template to standard output"))]
  CommitTemplate,
  #[structopt(about("Print possible values for `type` field of commit metadata"))]
  CommitTypes,
  #[structopt(about("Generate completion scripts"))]
  CompletionScripts,
  #[structopt(about("Diff generated content between commits"))]
  Diff,
  #[structopt(about("Generate readme"))]
  Readme,
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

  fs::write(&path, text).context(error::Filesystem { path })?;
}

#[throws]
fn clean_dir(path: impl AsRef<Path>) {
  let path = path.as_ref();

  info!("Cleaning `{}`…", path.display());

  if path.is_dir() {
    fs::remove_dir_all(path).context(error::Filesystem { path: &path })?;
  }

  fs::create_dir_all(path).context(error::Filesystem { path: &path })?;
}

impl Opt {
  #[throws]
  pub(crate) fn run(self) {
    let project = Project::load()?;

    match self {
      Self::Changelog => Self::changelog(&project)?,
      Self::CommitTemplate => {
        println!("{}", Metadata::default().to_string());
      }
      Self::CommitTypes => {
        for kind in Kind::VARIANTS {
          println!("{}", kind)
        }
      }
      Self::CompletionScripts => Self::completion_scripts(&project)?,
      Self::Readme => Self::readme(&project)?,
      Self::Book => Self::book(&project)?,
      Self::Man => Self::man(&project)?,
      Self::Diff => Self::diff(&project)?,
      Self::All => Self::all(&project)?,
    }
  }

  #[throws]
  pub(crate) fn all(project: &Project) {
    Self::changelog(&project)?;
    Self::completion_scripts(&project)?;
    Self::readme(&project)?;
    Self::book(&project)?;
    Self::man(&project)?;
  }

  #[throws]
  pub(crate) fn changelog(project: &Project) {
    info!("Generating changelog…");
    let changelog = Changelog::new(&project)?;

    let path = project.root.join("CHANGELOG.md");

    fs::write(&path, changelog.to_string()).context(error::Filesystem { path })?;
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
    .status_into_result()?
  }

  #[throws]
  pub(crate) fn diff(project: &Project) {
    let tmp = tempfile::tempdir().context(error::Tempdir)?;

    let generated = &[
      "/CHANGELOG.md",
      "/README.md",
      "/book/src/SUMMARY.md",
      "/book/src/bittorrent.md",
      "/book/src/commands.md",
      "/book/src/commands/*",
      "/book/src/faq.md",
      "/book/src/introduction.md",
      "/book/src/references.md",
      "/book/src/references/*",
      "/completions/*",
      "/man/*",
    ];

    let gen = |name: &str| -> Result<(), Error> {
      cmd!("cargo", "run", "--package", "gen", "all").status_into_result()?;

      let dir = tmp.path().join(name);

      fs::create_dir(&dir).context(error::Filesystem { path: &dir })?;

      let mut builder = OverrideBuilder::new(&project.root);

      for pattern in generated {
        builder.add(pattern)?;
      }

      let overrides = builder.build()?;

      for result in WalkDir::new(&project.root) {
        let entry = result?;

        let src = entry.path();

        if src.is_dir() {
          continue;
        }

        if !overrides.matched(&src, false).is_whitelist() {
          continue;
        }

        let relative = src
          .strip_prefix(&project.root)
          .context(error::StripPrefix)?;

        let dst = dir.join(relative);

        let dst_dir = dst.join("..").lexiclean();

        fs::create_dir_all(&dst_dir).context(error::Filesystem { path: dst_dir })?;

        fs::copy(&src, &dst).context(error::FilesystemCopy { src, dst })?;
      }

      Ok(())
    };

    const HEAD: &str = "HEAD";

    gen(HEAD)?;

    let head = project.repo.head()?.peel_to_commit()?;

    let parent = head.parent(0)?;

    let parent_hash = parent.id().to_string();

    cmd!("git", "checkout", &parent_hash).status_into_result()?;

    gen(&parent_hash)?;

    cmd!("diff", "-r", parent_hash, HEAD)
      .current_dir(tmp.path())
      .status_into_result()
      .ok();

    cmd!("git", "checkout", &head.id().to_string()).status_into_result()?;
  }

  #[throws]
  pub(crate) fn readme(project: &Project) {
    info!("Generating readme…");

    let template = project.root.join("bin/gen/templates/README.md");

    let readme = Readme::load(&project.config, &template)?;

    let text = readme.render_newline()?;

    let path = project.root.join("README.md");
    fs::write(&path, text).context(error::Filesystem { path })?;
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

      fs::write(&dst, page).context(error::Filesystem { path: dst })?;
    }

    clean_dir(&project.root.join("book/src/references/"))?;

    for section in &project.config.references {
      section.render_to(project.root.join("book/src").join(section.path()))?;
    }

    Faq::new(&project.config.faq).render_to(project.root.join("book/src/faq.md"))?;

    Summary::new(project).render_to(project.root.join("book/src/SUMMARY.md"))?;

    Introduction::new(&project.config).render_to(project.root.join("book/src/introduction.md"))?;
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

      fs::write(&dst, man).context(error::Filesystem { path: dst })?;
    }
  }
}
