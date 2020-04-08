use crate::common::*;

#[derive(Debug)]
pub(crate) struct Bin {
  pub(crate) bin: String,
  pub(crate) subcommands: Vec<Subcommand>,
}

impl Bin {
  #[throws]
  pub(crate) fn new(bin: &str) -> Self {
    let mut bin = Bin {
      bin: bin.into(),
      subcommands: Vec::new(),
    };

    bin.add_subcommands(&mut Vec::new())?;

    bin.subcommands.sort();

    bin
  }

  #[throws]
  fn add_subcommands(&mut self, command: &mut Vec<String>) {
    let subcommand = Subcommand::new(&self.bin, command.clone())?;

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
