use crate::common::*;

pub(crate) struct Project {
  pub(crate) repo: Repository,
  pub(crate) root: PathBuf,
  pub(crate) config: Config,
  pub(crate) bin: Bin,
}

impl Project {
  #[throws]
  pub(crate) fn load() -> Self {
    let start_dir = env::current_dir().context(error::CurrentDir)?;

    let repo = Repository::discover(&start_dir).context(error::RepositoryDiscover { start_dir })?;

    let root = repo
      .workdir()
      .ok_or_else(|| Error::Workdir {
        repo: repo.path().to_owned(),
      })?
      .to_owned();

    let config = Config::load(&root)?;

    let bin = Bin::new(&root.join("target/debug/imdl"))?;

    let example_commands = config
      .examples
      .iter()
      .map(|example| example.command.clone())
      .collect::<BTreeSet<String>>();

    let bin_commands = bin
      .subcommands
      .iter()
      .map(|subcommand| subcommand.command_line())
      .collect::<BTreeSet<String>>();

    if example_commands != bin_commands {
      throw!(Error::ExampleCommands {
        example: example_commands,
        bin: bin_commands
      });
    }

    Project {
      repo,
      root,
      config,
      bin,
    }
  }
}
