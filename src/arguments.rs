use crate::common::*;

#[derive(StructOpt)]
#[structopt(
  about(consts::ABOUT),
  version(consts::VERSION),
  author(consts::AUTHOR),
  help_message(consts::HELP_MESSAGE),
  version_message(consts::VERSION_MESSAGE),
  global_setting(AppSettings::ColoredHelp),
  global_setting(AppSettings::ColorAuto)
)]
pub(crate) struct Arguments {
  #[structopt(flatten)]
  options: Options,
  #[structopt(subcommand)]
  subcommand: Subcommand,
}

impl Arguments {
  pub(crate) fn run(self, env: &mut Env) -> Result<(), Error> {
    self.subcommand.run(env, &self.options)
  }

  pub(crate) fn options(&self) -> &Options {
    &self.options
  }
}
