use crate::common::*;

#[derive(StructOpt)]
#[structopt(
  help_message(consts::HELP_MESSAGE),
  version_message(consts::VERSION_MESSAGE),
  about("Verify files against a `.torrent` file.")
)]
pub(crate) struct Verify {
  #[structopt(
    name = "TORRENT",
    long = "metainfo",
    help = "Verify input data against `TORRENT` metainfo file.",
    parse(from_os_str)
  )]
  metainfo: PathBuf,
  #[structopt(
    name = "INPUT",
    long = "input",
    help = "Verify `INPUT`. Defaults to `info.name` field of torrent metainfo.",
    parse(from_os_str)
  )]
  input: Option<PathBuf>,
}

impl Verify {
  pub(crate) fn run(self, env: &mut Env) -> Result<(), Error> {
    let metainfo_path = env.resolve(&self.metainfo);
    let metainfo = Metainfo::load(&metainfo_path)?;

    let base = if let Some(input) = &self.input {
      env.resolve(input)
    } else {
      metainfo_path.parent().unwrap().join(&metainfo.info.name)
    };

    let status = metainfo.verify(&base)?;

    status.write(env)?;

    if status.good() {
      Ok(())
    } else {
      Err(Error::Verify { status })
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn environment(args: &[&str]) -> TestEnv {
    testing::env(["torrent", "create"].iter().chain(args).cloned())
  }

  #[test]
  fn require_metainfo_argument() {
    let mut env = environment(&[]);
    assert!(matches!(env.run(), Err(Error::Clap { .. })));
  }
}
