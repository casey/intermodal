use crate::common::*;

#[derive(StructOpt)]
pub(crate) struct Opt {
  #[structopt(long = "unstable", short = "u")]
  unstable: bool,
  #[structopt(subcommand)]
  subcommand: Subcommand,
}

impl Opt {
  pub(crate) fn run(self, env: &mut Env) -> Result<(), Error> {
    self.subcommand.run(env, self.unstable)
  }
}
