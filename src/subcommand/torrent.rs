use crate::common::*;

mod announce;
mod create;
mod dump;
mod from_link;
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
  Announce(announce::Announce),
  Create(create::Create),
  Dump(dump::Dump),
  FromLink(from_link::FromLink),
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
      Self::Announce(announce) => announce.run(env),
      Self::Create(create) => create.run(env, options),
      Self::Dump(dump) => dump.run(env),
      Self::FromLink(from_link) => from_link.run(env, options),
      Self::Link(link) => link.run(env),
      Self::PieceLength(piece_length) => piece_length.run(env),
      Self::Show(show) => show.run(env),
      Self::Stats(stats) => stats.run(env, options),
      Self::Verify(verify) => verify.run(env, options),
    }
  }
}
