use crate::common::*;

#[derive(StructOpt)]
pub(crate) enum Opt {
  IssueTemplate,
  Types,
  Update,
}

impl Opt {
  #[throws]
  pub(crate) fn run(self) {
    match self {
      Self::Types => {
        for kind in Kind::VARIANTS {
          println!("{}", kind)
        }
      }
      Self::IssueTemplate => {
        println!("{}", Metadata::default().to_string());
      }
      Self::Update => {
        let cwd = env::current_dir()?;

        let repo = Repository::discover(cwd)?;

        let changelog = Changelog::new(&repo)?;

        let dst = repo.workdir().unwrap().join("CHANGELOG.md");

        fs::write(dst, changelog.to_string())?;
      }
    }
  }
}
