use crate::common::*;

#[derive(StructOpt)]
pub(crate) struct Arguments {
  #[structopt(flatten)]
  options: Options,
  #[structopt(subcommand)]
  subcommand: Subcommand,
}

impl Arguments {
  #[throws]
  pub(crate) fn run(self) {
    self.subcommand.run(self.options)?;
  }
}
