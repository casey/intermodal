use crate::common::*;

mod create;
mod stats;

#[derive(StructOpt)]
#[structopt(
  help_message(consts::HELP_MESSAGE),
  version_message(consts::VERSION_MESSAGE),
  about("Subcommands related to the BitTorrent protocol.")
)]
pub(crate) enum Torrent {
  Create(torrent::create::Create),
  Stats(torrent::stats::Stats),
}

impl Torrent {
  pub(crate) fn run(self, env: &mut Env, unstable: bool) -> Result<(), Error> {
    match self {
      Self::Create(create) => create.run(env),
      Self::Stats(stats) => stats.run(env, unstable),
    }
  }
}