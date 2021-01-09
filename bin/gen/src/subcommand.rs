use crate::common::*;

#[derive(StructOpt)]
pub(crate) enum Subcommand {
  #[structopt(about("Update all generated docs"))]
  All {
    #[structopt(long = "no-changelog", help = "Don't generate the changelog.")]
    no_changelog: bool,
  },
  #[structopt(about("Generate book"))]
  Book {
    #[structopt(long = "no-changelog", help = "Don't generate the changelog.")]
    no_changelog: bool,
  },
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

impl Subcommand {
  #[throws]
  pub(crate) fn run(self, options: Options) {
    let project = Project::load(&options)?;

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
      Self::Book { no_changelog } => Self::book(&project, no_changelog)?,
      Self::Man => Self::man(&project)?,
      Self::Diff => Self::diff(&project)?,
      Self::All { no_changelog } => Self::all(&project, no_changelog)?,
    }
  }

  #[throws]
  pub(crate) fn all(project: &Project, no_changelog: bool) {
    if !no_changelog {
      Self::changelog(&project)?;
    }
    Self::completion_scripts(&project)?;
    Self::readme(&project)?;
    Self::book(&project, no_changelog)?;
    Self::man(&project)?;
  }

  #[throws]
  pub(crate) fn changelog(project: &Project) {
    info!("Generating changelog…");
    let changelog = Changelog::new(&project)?;

    let path = project.gen()?.join("CHANGELOG.md");

    fs::write(&path, changelog.render(false)?).context(error::Filesystem { path })?;
  }

  #[throws]
  pub(crate) fn completion_scripts(project: &Project) {
    info!("Generating completion scripts…");
    let completions = project.gen()?.join("completions");

    clean_dir(&completions)?;

    cmd!(&project.executable, "completions", "--dir", completions).status_into_result()?
  }

  #[throws]
  pub(crate) fn diff(project: &Project) {
    let tmp = tempfile::tempdir().context(error::Tempdir)?;

    let gen = |name: &str| -> Result<(), Error> {
      let src = Path::new("target/gen");

      fs::remove_dir_all(src).context(error::Filesystem { path: src })?;

      cmd!("cargo", "run", "--package", "gen", "all").status_into_result()?;

      let dir = tmp.path().join(name);

      fs::create_dir(&dir).context(error::Filesystem { path: &dir })?;

      fs_extra::dir::copy(src, &dir, &fs_extra::dir::CopyOptions::new())
        .context(error::FilesystemRecursiveCopy { src, dst: dir })?;

      Ok(())
    };

    const HEAD: &str = "HEAD";

    gen(HEAD)?;

    let repo = project.repo()?;

    let head = repo.head()?;

    let head_commit = head.peel_to_commit()?;

    let parent = head_commit.parent(0)?;

    let parent_hash = parent.id().to_string();

    cmd!("git", "checkout", &parent_hash).status_into_result()?;

    gen(&parent_hash)?;

    cmd!("colordiff", "-ur", parent_hash, HEAD)
      .current_dir(tmp.path())
      .status_into_result()
      .ok();

    cmd!(
      "git",
      "checkout",
      head
        .shorthand()
        .map(str::to_owned)
        .unwrap_or_else(|| head_commit.id().to_string())
    )
    .status_into_result()?;
  }

  #[throws]
  pub(crate) fn readme(project: &Project) {
    info!("Generating readme…");

    let template = project.root.join("bin/gen/templates/README.md");

    let readme = Readme::load(&project.config, &template)?;

    let text = readme.render_newline()?;

    let path = project.gen()?.join("README.md");
    fs::write(&path, &text).context(error::Filesystem { path })?;

    let path = project.root.join("README.md");
    fs::write(&path, &text).context(error::Filesystem { path })?;
  }

  #[throws]
  pub(crate) fn book(project: &Project, no_changelog: bool) {
    info!("Generating book…");

    let gen = project.gen()?;

    let out = gen.join("book");

    fs::create_dir_all(&out).context(error::Filesystem { path: &out })?;

    blank(out.join("commands.md"), "Commands")?;
    blank(out.join("bittorrent.md"), "BitTorrent")?;
    blank(out.join("references.md"), "References")?;

    let commands = out.join("commands");

    clean_dir(&commands)?;

    for subcommand in &project.bin.subcommands {
      let page = subcommand.page()?;

      let dst = commands.join(format!("{}.md", subcommand.slug()));

      fs::write(&dst, page).context(error::Filesystem { path: dst })?;
    }

    clean_dir(&out.join("references"))?;

    for section in &project.config.references {
      section.render_to(out.join(section.path()))?;
    }

    Faq::new(&project.config.faq).render_to(out.join("faq.md"))?;

    Introduction::new(&project.config).render_to(out.join("introduction.md"))?;

    let include_changelog = !no_changelog;

    Summary::new(project, include_changelog).render_to(out.join("SUMMARY.md"))?;

    if !no_changelog {
      let changelog = Changelog::new(&project)?;
      let dst = out.join("changelog.md");
      fs::write(&dst, changelog.render(true)?).context(error::Filesystem { path: dst })?;
    }
  }

  #[throws]
  pub(crate) fn man(project: &Project) {
    info!("Generating man pages…");
    let mans = project.gen()?.join("man");

    clean_dir(&mans)?;

    for subcommand in &project.bin.subcommands {
      let man = subcommand.man()?;

      let dst = mans.join(format!("{}.1", subcommand.slug()));

      info!("Writing man page to `{}`", dst.display());

      fs::write(&dst, man).context(error::Filesystem { path: dst })?;
    }
  }
}
