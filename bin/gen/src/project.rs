use crate::common::*;

pub(crate) struct Project {
  pub(crate) root: PathBuf,
  pub(crate) config: Config,
  pub(crate) bin: Bin,
  pub(crate) executable: PathBuf,
}

impl Project {
  #[throws]
  pub(crate) fn load(options: &Options) -> Self {
    let root = env::current_dir().context(error::CurrentDir)?;

    let config = Config::load(&root)?;

    let bin = Bin::new(&options.bin)?;

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
      executable: options.bin.clone(),
      bin,
      config,
      root,
    }
  }

  #[throws]
  pub(crate) fn gen(&self) -> PathBuf {
    let gen = self.root.join("target").join("gen");

    if !gen.is_dir() {
      fs::create_dir_all(&gen).context(error::Filesystem { path: &gen })?;
    }

    gen
  }

  #[throws]
  pub(crate) fn repo(&self) -> Repository {
    Repository::discover(&self.root).context(error::RepositoryDiscover {
      start_dir: &self.root,
    })?
  }
}
