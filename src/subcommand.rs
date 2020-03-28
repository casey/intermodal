use crate::common::*;

mod completions;
mod torrent;

#[derive(StructOpt)]
pub(crate) enum Subcommand {
  Torrent(torrent::Torrent),
  Completions(completions::Completions),
}

impl Subcommand {
  pub(crate) fn run(self, env: &mut Env, options: &Options) -> Result<(), Error> {
    match self {
      Self::Torrent(torrent) => torrent.run(env, options),
      Self::Completions(completions) => completions.run(env),
    }
  }
}
