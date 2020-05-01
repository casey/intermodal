use crate::common::*;

pub(crate) struct Bin {
  path: PathBuf,
  pub(crate) subcommands: Vec<BinSubcommand>,
}

impl Bin {
  #[throws]
  pub(crate) fn new(path: &Path) -> Bin {
    let mut bin = Bin {
      path: path.into(),
      subcommands: Vec::new(),
    };

    bin.add_subcommands(&mut Vec::new())?;

    bin.subcommands.sort();

    bin
  }

  #[throws]
  fn add_subcommands(&mut self, command: &mut Vec<String>) {
    let subcommand = BinSubcommand::new(&self.path, command.clone())?;

    for name in &subcommand.subcommands {
      command.push(name.into());
      if name != "help" {
        self.add_subcommands(command)?;
      }
      command.pop();
    }

    self.subcommands.push(subcommand);
  }
}
