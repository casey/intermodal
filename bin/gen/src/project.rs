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
    let repo = Repository::discover(env::current_dir()?)?;

    let root = repo
      .workdir()
      .ok_or_else(|| anyhow!("Repository at `{}` had no workdir", repo.path().display()))?
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
      println!("Example commands:");
      for command in example_commands {
        println!("{}", command);
      }

      println!("…don't match bin commands:");
      for command in bin_commands {
        println!("{}", command);
      }

      throw!(anyhow!(""));
    }

    Project {
      repo,
      root,
      config,
      bin,
    }
  }
}
