use crate::common::*;

mod create;
mod stats;

#[derive(StructOpt)]
pub(crate) enum Torrent {
  Create(torrent::create::Create),
  Stats(torrent::stats::Stats),
}

impl Torrent {
  pub(crate) fn run(self, env: &mut Environment, unstable: bool) -> Result<(), Error> {
    match self {
      Self::Create(create) => create.run(env),
      Self::Stats(stats) => stats.run(env, unstable),
    }
  }
}
