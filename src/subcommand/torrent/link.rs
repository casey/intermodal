use crate::common::*;

const INPUT_HELP: &str = "Generate magnet link from metainfo at `INPUT`. If `INPUT` is `-`, read \
                          metainfo from standard input.";

const INPUT_FLAG: &str = "input-flag";

const INPUT_POSITIONAL: &str = "<INPUT>";

#[derive(StructOpt)]
#[structopt(
  help_message(consts::HELP_MESSAGE),
  version_message(consts::VERSION_MESSAGE),
  about("Generate a magnet link from a .torrent file.")
)]
pub(crate) struct Link {
  #[structopt(
    name = INPUT_FLAG,
    long = "input",
    short = "i",
    value_name = "INPUT",
    empty_values(false),
    parse(try_from_os_str = InputTarget::try_from_os_str),
    help = INPUT_HELP,
  )]
  input_flag: Option<InputTarget>,
  #[structopt(
    name = INPUT_POSITIONAL,
    value_name = "INPUT",
    empty_values(false),
    parse(try_from_os_str = InputTarget::try_from_os_str),
    required_unless = INPUT_FLAG,
    conflicts_with = INPUT_FLAG,
    help = INPUT_HELP,
  )]
  input_positional: Option<InputTarget>,
  #[structopt(
    long = "open",
    short = "O",
    help = "Open generated magnet link. Uses `xdg-open`, `gnome-open`, or `kde-open` on Linux; \
            `open` on macOS; and `cmd /C start` on Windows."
  )]
  open: bool,
  #[structopt(
    long = "peer",
    short = "p",
    value_name = "PEER",
    help = "Add `PEER` to magnet link."
  )]
  peers: Vec<HostPort>,
  #[structopt(
    long = "select-only",
    short = "s",
    value_name = "INDICES",
    use_delimiter = true,
    help = "Select files to download. Values are indices into the `info.files` list, e.g. \
            `--select-only 1,2,3`."
  )]
  indices: Vec<u64>,
}

impl Link {
  pub(crate) fn run(self, env: &mut Env) -> Result<(), Error> {
    let input = xor_args(
      "input_flag",
      &self.input_flag,
      "input_positional",
      &self.input_positional,
    )?;

    let input = env.read(input)?;

    let infohash = Infohash::from_input(&input)?;
    let metainfo = Metainfo::from_input(&input)?;

    let mut link = MagnetLink::with_infohash(infohash);

    link.set_name(&metainfo.info.name);

    for result in metainfo.trackers() {
      link.add_tracker(result?);
    }

    for peer in self.peers {
      link.add_peer(peer);
    }

    for index in self.indices {
      link.add_index(index);
    }

    let url = link.to_url();

    outln!(env, "{}", url)?;

    if self.open {
      Platform::open_url(&url)?;
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use pretty_assertions::assert_eq;

  #[test]
  fn input_required() {
    test_env! {
      args: [
        "torrent",
        "link",
      ],
      tree: {
      },
      matches: Err(Error::Clap { .. }),
    };

    test_env! {
      args: [
        "torrent",
        "link",
        "--input",
        "foo",
      ],
      tree: {
      },
      matches: Err(Error::Filesystem { .. }),
    };

    test_env! {
      args: [
        "torrent",
        "link",
        "foo",
      ],
      tree: {
      },
      matches: Err(Error::Filesystem { .. }),
    };

    test_env! {
      args: [
        "torrent",
        "link",
        "--input",
        "foo",
        "foo",
      ],
      tree: {
      },
      matches: Err(Error::Clap { .. }),
    };
  }

  #[test]
  fn no_announce_flag() {
    const INFO: &str = "d6:lengthi0e4:name3:foo12:piece lengthi1e6:pieces0:e";

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

    env.assert_ok();

    let infohash = Sha1Digest::from_data(INFO.as_bytes());

    assert_eq!(
      env.out(),
      format!("magnet:?xt=urn:btih:{}&dn=foo\n", infohash),
    );
  }

  #[test]
  fn no_announce_positional() {
    const INFO: &str = "d6:lengthi0e4:name3:foo12:piece lengthi1e6:pieces0:e";

    let mut env = test_env! {
      args: [
        "torrent",
        "link",
        "foo.torrent",
      ],
      tree: {
        "foo.torrent": "d4:infod6:lengthi0e4:name3:foo12:piece lengthi1e6:pieces0:ee",
      }
    };

    env.assert_ok();

    let infohash = Sha1Digest::from_data(INFO.as_bytes());

    assert_eq!(
      env.out(),
      format!("magnet:?xt=urn:btih:{}&dn=foo\n", infohash),
    );
  }

  #[test]
  fn with_announce() {
    const INFO: &str = "d6:lengthi0e4:name3:foo12:piece lengthi1e6:pieces0:e";

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

    env.assert_ok();

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
    const INFO: &str = "d6:lengthi0e4:name3:foo12:piece lengthi1e6:pieces0:e";

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

    env.assert_ok();

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
    const INFO: &str = "d6:lengthi0e4:name3:foo12:piece lengthi1e6:pieces0:e";

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

    env.assert_ok();

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
  fn with_indices() {
    const INFO: &str = "d6:lengthi0e4:name3:foo12:piece lengthi1e6:pieces0:e";

    let mut env = test_env! {
      args: [
        "torrent",
        "link",
        "--input",
        "foo.torrent",
        "--select-only",
        "2,4",
        "--select-only",
        "4,6",
      ],
      tree: {
        "foo.torrent": "d\
          8:announce24:https://foo.com/announce\
          4:infod6:lengthi0e4:name3:foo12:piece lengthi1e6:pieces0:e\
        e",
      }
    };

    env.assert_ok();

    let infohash = Sha1Digest::from_data(INFO.as_bytes());

    assert_eq!(
      env.out(),
      format!(
        "magnet:?xt=urn:btih:{}&dn=foo&tr=https://foo.com/announce&so=2,4,6\n",
        infohash
      ),
    );
  }

  #[test]
  fn infohash_correct_with_nonstandard_info_dict() {
    const INFO: &str = "d1:ai0e6:lengthi0e4:name3:foo12:piece lengthi1e6:pieces0:e";

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

    env.assert_ok();

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
      env.run(), Err(Error::MetainfoValidate { input, source: MetainfoError::Type })
      if input == "foo.torrent"
    );
  }
}
