use crate::common::*;

mod create;
mod piece_length;
mod show;
mod stats;

#[derive(StructOpt)]
#[structopt(
  help_message(consts::HELP_MESSAGE),
  version_message(consts::VERSION_MESSAGE),
  about("Subcommands related to the BitTorrent protocol.")
)]
pub(crate) enum Torrent {
  Create(create::Create),
  #[structopt(alias = "piece-size")]
  PieceLength(piece_length::PieceLength),
  Show(show::Show),
  Stats(stats::Stats),
}

impl Torrent {
  pub(crate) fn run(self, env: &mut Env, unstable: bool) -> Result<(), Error> {
    match self {
      Self::Create(create) => create.run(env),
      Self::PieceLength(piece_length) => piece_length.run(env),
      Self::Show(show) => show.run(env),
      Self::Stats(stats) => stats.run(env, unstable),
    }
  }
}
