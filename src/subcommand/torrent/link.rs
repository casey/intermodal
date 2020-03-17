use crate::common::*;

#[derive(StructOpt)]
#[structopt(
  help_message(consts::HELP_MESSAGE),
  version_message(consts::VERSION_MESSAGE),
  about("Generate a magnet link from a `.torrent` file.")
)]
pub(crate) struct Link {
  #[structopt(
    long = "input",
    short = "i",
    value_name = "METAINFO",
    help = "Generate magnet link from metainfo at `PATH`.",
    parse(from_os_str)
  )]
  input: PathBuf,
}

impl Link {
  pub(crate) fn run(self, env: &mut Env) -> Result<(), Error> {
    let input = env.resolve(&self.input);
    let infohash = Infohash::load(&input)?;
    let metainfo = Metainfo::load(&input)?;

    let mut link = MagnetLink::with_infohash(infohash);

    let mut trackers = HashSet::new();
    for result in metainfo.trackers() {
      let tracker = result?;
      if !trackers.contains(&tracker) {
        trackers.insert(tracker.clone());
        link.add_tracker(tracker);
      }
    }

    outln!(env, "{}", link.to_url())?;

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use claim::assert_ok;
  use pretty_assertions::assert_eq;

  #[test]
  fn no_announce() {
    let mut env = test_env! {
      args: [
        "torrent",
        "link",
        "--input",
        "foo.torrent",
      ],
      tree: {
        "foo.torrent": "d4:infod6:lengthi0e4:name3:foo12:piece lengthi1e6:pieces0:ee",
      }
    };

    assert_ok!(env.run());

    const INFO: &str = "d6:lengthi0e4:name3:foo12:piece lengthi1e6:pieces0:e";

    let infohash = Sha1Digest::from_data(INFO.as_bytes());

    assert_eq!(env.out(), format!("magnet:?xt=urn:btih:{}\n", infohash),);
  }

  #[test]
  fn with_announce() {
    let mut env = test_env! {
      args: [
        "torrent",
        "link",
        "--input",
        "foo.torrent",
      ],
      tree: {
        "foo.torrent": "d8:announce24:https://foo.com/announce4:infod6:lengthi0e4:name3:foo12:piece lengthi1e6:pieces0:ee",
      }
    };

    assert_ok!(env.run());

    const INFO: &str = "d6:lengthi0e4:name3:foo12:piece lengthi1e6:pieces0:e";

    let infohash = Sha1Digest::from_data(INFO.as_bytes());

    assert_eq!(
      env.out(),
      format!(
        "magnet:?xt=urn:btih:{}&tr=https://foo.com/announce\n",
        infohash
      ),
    );
  }

  #[test]
  fn infohash_correct_with_nonstandard_info_dict() {
    let mut env = test_env! {
      args: [
        "torrent",
        "link",
        "--input",
        "foo.torrent",
      ],
      tree: {
        "foo.torrent": "d4:infod1:ai0e6:lengthi0e4:name3:foo12:piece lengthi1e6:pieces0:ee",
      }
    };

    assert_ok!(env.run());

    const INFO: &str = "d1:ai0e6:lengthi0e4:name3:foo12:piece lengthi1e6:pieces0:e";

    let infohash = Sha1Digest::from_data(INFO.as_bytes());

    assert_eq!(env.out(), format!("magnet:?xt=urn:btih:{}\n", infohash),);
  }

  #[test]
  #[ignore]
  fn bad_metainfo_error() {
    todo!()
  }

  #[test]
  #[ignore]
  fn trailing_bytes_error() {
    todo!()
  }
}
