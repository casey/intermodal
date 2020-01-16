use crate::common::*;

use structopt::clap::{AppSettings, ArgSettings};

#[derive(StructOpt)]
#[structopt(
  about(consts::ABOUT),
  version(consts::VERSION),
  author(consts::AUTHOR),
  global_setting(AppSettings::ColoredHelp),
  global_setting(AppSettings::ColorAuto)
)]
pub(crate) struct Opt {
  #[structopt(long = "unstable", short = "u")]
  unstable: bool,
  #[structopt(
    long = "color",
    default_value = use_color::AUTO,
    set = ArgSettings::CaseInsensitive,
    possible_values = use_color::VALUES,
  )]
  pub(crate) use_color: UseColor,
  #[structopt(subcommand)]
  subcommand: Subcommand,
}

impl Opt {
  pub(crate) fn run(self, env: &mut Env) -> Result<(), Error> {
    self.subcommand.run(env, self.unstable)
  }
}
