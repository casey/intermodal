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
  #[structopt(
    long = "peer",
    short = "p",
    value_name = "PEER",
    help = "Add PEER to magnet link."
  )]
  peers: Vec<HostPort>,
}

impl Link {
  pub(crate) fn run(self, env: &mut Env) -> Result<(), Error> {
    let input = env.resolve(&self.input);
    let infohash = Infohash::load(&input)?;
    let metainfo = Metainfo::load(&input)?;

    let mut link = MagnetLink::with_infohash(infohash);

    link.set_name(&metainfo.info.name);

    let mut trackers = HashSet::new();
    for result in metainfo.trackers() {
      let tracker = result?;
      if !trackers.contains(&tracker) {
        trackers.insert(tracker.clone());
        link.add_tracker(tracker);
      }
    }

    for peer in self.peers {
      link.add_peer(peer);
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

    assert_eq!(
      env.out(),
      format!("magnet:?xt=urn:btih:{}&dn=foo\n", infohash),
    );
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
        "foo.torrent": "d\
          8:announce24:https://foo.com/announce\
          4:infod6:lengthi0e4:name3:foo12:piece lengthi1e6:pieces0:e\
        e",
      }
    };

    assert_ok!(env.run());

    const INFO: &str = "d6:lengthi0e4:name3:foo12:piece lengthi1e6:pieces0:e";

    let infohash = Sha1Digest::from_data(INFO.as_bytes());

    assert_eq!(
      env.out(),
      format!(
        "magnet:?xt=urn:btih:{}&dn=foo&tr=https://foo.com/announce\n",
        infohash
      ),
    );
  }

  #[test]
  fn unique_trackers() {
    let mut env = test_env! {
      args: [
        "torrent",
        "link",
        "--input",
        "foo.torrent",
      ],
      tree: {
        "foo.torrent": "d\
          8:announce24:https://foo.com/announce\
          13:announce-listll24:https://foo.com/announceel24:https://bar.com/announceee\
          4:infod6:lengthi0e4:name3:foo12:piece lengthi1e6:pieces0:e\
        e",
      }
    };

    assert_ok!(env.run());

    const INFO: &str = "d6:lengthi0e4:name3:foo12:piece lengthi1e6:pieces0:e";

    let infohash = Sha1Digest::from_data(INFO.as_bytes());

    assert_eq!(
      env.out(),
      format!(
        "magnet:?xt=urn:btih:{}&dn=foo&tr=https://foo.com/announce&tr=https://bar.com/announce\n",
        infohash
      ),
    );
  }
  #[test]
  fn with_peer() {
    let mut env = test_env! {
      args: [
        "torrent",
        "link",
        "--input",
        "foo.torrent",
        "--peer",
        "foo.com:1337",
      ],
      tree: {
        "foo.torrent": "d\
          8:announce24:https://foo.com/announce\
          4:infod6:lengthi0e4:name3:foo12:piece lengthi1e6:pieces0:e\
        e",
      }
    };

    assert_ok!(env.run());

    const INFO: &str = "d6:lengthi0e4:name3:foo12:piece lengthi1e6:pieces0:e";

    let infohash = Sha1Digest::from_data(INFO.as_bytes());

    assert_eq!(
      env.out(),
      format!(
        "magnet:?xt=urn:btih:{}&dn=foo&tr=https://foo.com/announce&x.pe=foo.com:1337\n",
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

    assert_eq!(
      env.out(),
      format!("magnet:?xt=urn:btih:{}&dn=foo\n", infohash),
    );
  }

  #[test]
  fn bad_metainfo_error() {
    let mut env = test_env! {
      args: [
        "torrent",
        "link",
        "--input",
        "foo.torrent",
      ],
      tree: {
        "foo.torrent": "i0e",
      }
    };

    assert_matches!(
      env.run(), Err(Error::MetainfoValidate { path, source: MetainfoError::Type })
      if path == env.resolve("foo.torrent")
    );
  }
}
