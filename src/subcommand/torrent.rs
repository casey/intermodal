use crate::common::*;

mod create;
mod link;
mod piece_length;
mod show;
mod stats;
mod verify;

#[derive(StructOpt)]
#[structopt(
  help_message(consts::HELP_MESSAGE),
  version_message(consts::VERSION_MESSAGE),
  about("Subcommands related to the BitTorrent protocol.")
)]
pub(crate) enum Torrent {
  Create(create::Create),
  Link(link::Link),
  #[structopt(alias = "piece-size")]
  PieceLength(piece_length::PieceLength),
  Show(show::Show),
  Stats(stats::Stats),
  Verify(verify::Verify),
}

impl Torrent {
  pub(crate) fn run(self, env: &mut Env, options: &Options) -> Result<(), Error> {
    match self {
      Self::Create(create) => create.run(env, options),
      Self::Link(link) => link.run(env),
      Self::PieceLength(piece_length) => piece_length.run(env),
      Self::Show(show) => show.run(env),
      Self::Stats(stats) => stats.run(env, options),
      Self::Verify(verify) => verify.run(env, options),
    }
  }
}
