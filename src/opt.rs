use crate::common::*;

use structopt::clap::AppSettings;

#[derive(StructOpt)]
#[structopt(
  about(consts::ABOUT),
  version(consts::VERSION),
  author(consts::AUTHOR),
  setting(AppSettings::ColoredHelp),
  setting(AppSettings::ColorAuto)
)]
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
