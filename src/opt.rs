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
pub(crate) struct Opt {
  #[structopt(
    long = "unstable",
    short = "u",
    help = "Enable unstable features.",
    long_help = "Enable unstable features. To avoid premature stabilization and excessive version churn, unstable features are unavailable unless this flag is set. Unstable features are not bound by semantic versioning stability guarantees, and may be changed or removed at any time."
  )]
  unstable: bool,
  #[structopt(
    long = "color",
    default_value = use_color::AUTO,
    set = ArgSettings::CaseInsensitive,
    possible_values = use_color::VALUES,
    help = "Print colorful output.",
    long_help = "Print colorful output. When `auto`, the default, colored output is only enabled if imdl detects that it is connected to a terminal, the `NO_COLOR` environment variable is not set, and the `TERM` environment variable is not set with a value of `dumb`.",
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
