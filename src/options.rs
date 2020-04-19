use crate::common::*;

#[derive(StructOpt)]
pub(crate) struct Options {
  #[structopt(long = "quiet", short = "q", help = "Suppress normal output.")]
  pub(crate) quiet: bool,
  #[structopt(
    long = "unstable",
    short = "u",
    help = "Enable unstable features. To avoid premature stabilization and excessive version \
            churn, unstable features are unavailable unless this flag is set. Unstable features \
            are not bound by semantic versioning stability guarantees, and may be changed or \
            removed at any time."
  )]
  unstable: bool,
  #[structopt(
    long = "color",
    short = "c",
    value_name = "WHEN",
    default_value = UseColor::Auto.into(),
    possible_values = UseColor::VARIANTS,
    help = "Print colorful output according to `WHEN`. When `auto`, the default, colored output \
            is only enabled if imdl detects that it is connected to a terminal, the `NO_COLOR` \
            environment variable is not set, and the `TERM` environment variable is not set to \
            `dumb`.",
  )]
  pub(crate) use_color: UseColor,
  #[structopt(
    long = "terminal",
    short = "t",
    help = "Disable automatic terminal detection and behave as if both standard output and \
            standard error are connected to a terminal."
  )]
  pub(crate) terminal: bool,
}

impl Options {
  pub(crate) fn require_unstable(&self, feature: &'static str) -> Result<(), Error> {
    if self.unstable {
      Ok(())
    } else {
      Err(Error::Unstable { feature })
    }
  }
}
