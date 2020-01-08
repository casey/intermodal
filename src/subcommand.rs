use crate::common::*;

#[derive(StructOpt)]
pub(crate) enum Subcommand {
  Torrent(Torrent),
}

impl Subcommand {
  pub(crate) fn run(self, env: &mut Env, unstable: bool) -> Result<(), Error> {
    match self {
      Self::Torrent(torrent) => torrent.run(env, unstable),
    }
  }
}
