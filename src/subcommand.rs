use crate::common::*;

mod torrent;

#[derive(StructOpt)]
pub(crate) enum Subcommand {
  Torrent(torrent::Torrent),
}

impl Subcommand {
  pub(crate) fn run(self, env: &mut Env, options: &Options) -> Result<(), Error> {
    match self {
      Self::Torrent(torrent) => torrent.run(env, options),
    }
  }
}
