use crate::common::*;

const INPUT_HELP: &str =
  "Read torrent metainfo from `INPUT`. If `INPUT` is `-`, read metainfo from standard input.";

const INPUT_FLAG: &str = "input-flag";

const INPUT_POSITIONAL: &str = "<INPUT>";

#[derive(StructOpt)]
#[structopt(
  help_message(consts::HELP_MESSAGE),
  version_message(consts::VERSION_MESSAGE),
  about("Announce a .torrent file.")
)]
pub(crate) struct Announce {
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
}

impl Announce {
  pub(crate) fn run(self, env: &mut Env) -> Result<(), Error> {
    let target = xor_args(
      "input_flag",
      &self.input_flag,
      "input_positional",
      &self.input_positional,
    )?;

    let input = env.read(target)?;
    let infohash = Infohash::from_input(&input)?;
    let metainfo = Metainfo::from_input(&input)?;
    let mut peers = HashSet::new();
    let mut usable_trackers = 0;

    for tracker_url in metainfo.trackers() {
      let tracker_url = match tracker_url {
        Ok(tracker_url) => tracker_url,
        Err(err) => {
          errln!(env, "Skipping tracker: {}", err)?;
          continue;
        }
      };

      let client = match tracker::Client::from_url(&tracker_url) {
        Ok(client) => client,
        Err(err) => {
          errln!(env, "Couldn't build tracker client. {}", err)?;
          continue;
        }
      };

      usable_trackers += 1;
      match client.announce_exchange(&infohash) {
        Ok(peer_list) => peers.extend(peer_list),
        Err(err) => errln!(env, "Announce failed: {}", err)?,
      }
    }

    if usable_trackers == 0 {
      return Err(Error::MetainfoMissingTrackers);
    }

    for peer in &peers {
      outln!(env, "{}", peer)?;
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[cfg(test)]
  pub(crate) fn new_dummy_metainfo() -> Metainfo {
    Metainfo {
      announce: None,
      announce_list: None,
      nodes: None,
      comment: None,
      created_by: None,
      creation_date: None,
      encoding: None,
      info: Info {
        private: None,
        piece_length: Bytes(16 * 1024),
        source: None,
        name: "testing".into(),
        pieces: PieceList::from_pieces(["test", "data"]),
        mode: Mode::Single {
          length: Bytes(2 * 16 * 1024),
          md5sum: None,
        },
        update_url: None,
      },
    }
  }

  #[test]
  fn input_required() {
    test_env! {
      args: [
        "torrent",
        "announce",
      ],
      tree: {
      },
      matches: Err(Error::Clap { .. }),
    };
  }

  #[test]
  fn input_arguments_positional() {
    let mut env = test_env! {
      args: [
        "torrent",
        "announce",
        "foo",
      ],
      tree: {},
    };
    assert_matches!(env.run(), Err(error::Error::Filesystem { .. }));
  }

  #[test]
  fn input_arguments_flag() {
    let mut env = test_env! {
      args: [
        "torrent",
        "announce",
        "--input",
        "foo",
      ],
      tree: {},
    };
    assert_matches!(env.run(), Err(error::Error::Filesystem { .. }));
  }

  #[test]
  fn input_arguments_conflict() {
    let mut env = test_env! {
      args: [
        "torrent",
        "announce",
        "--input",
        "foo",
        "bar",
      ],
      tree: {},
    };
    assert_matches!(env.run(), Err(Error::Clap { .. }));
  }

  #[test]
  fn metainfo_missing_trackers() {
    let mut env = test_env! {
      args: [
        "torrent",
        "announce",
        "--input",
        "test.torrent",
      ],
      tree: {},
    };
    let metainfo = new_dummy_metainfo();

    env.write("test.torrent", metainfo.serialize().unwrap());
    assert_matches!(env.run(), Err(Error::MetainfoMissingTrackers));
  }

  #[test]
  fn metainfo_no_udp_trackers() {
    let mut env = test_env! {
      args: [
        "torrent",
        "announce",
        "--input",
        "test.torrent",
      ],
      tree: {},
    };
    let https_tracker_url = "utp://intermodal.io:443/tracker/announce";
    let metainfo = Metainfo {
      announce: None,
      announce_list: Some(vec![vec![https_tracker_url.into()]]),
      nodes: None,
      comment: None,
      created_by: None,
      creation_date: None,
      encoding: None,
      info: Info {
        private: None,
        piece_length: Bytes(16 * 1024),
        source: None,
        name: "testing".into(),
        pieces: PieceList::from_pieces(["test", "data"]),
        mode: Mode::Single {
          length: Bytes(2 * 16 * 1024),
          md5sum: None,
        },
        update_url: None,
      },
    };

    env.write("test.torrent", metainfo.serialize().unwrap());
    assert_matches!(env.run(), Err(Error::MetainfoMissingTrackers));
    assert_eq!(
      env.err(),
      format!(
        "Couldn't build tracker client. Cannot connect to tracker `{https_tracker_url}`: only UDP trackers are supported\n",
      )
    );
  }

  #[test]
  fn tracker_host_port_not_well_formed() {
    let mut env = test_env! {
      args: [
        "torrent",
        "announce",
        "--input",
        "test.torrent",
      ],
      tree: {},
    };
    let tracker_url = "udp://1.2.3.4:1333337/announce";
    let metainfo = Metainfo {
      announce: None,
      announce_list: Some(vec![vec![tracker_url.into()]]),
      nodes: None,
      comment: None,
      created_by: None,
      creation_date: None,
      encoding: None,
      info: Info {
        private: None,
        piece_length: Bytes(16 * 1024),
        source: None,
        name: "testing".into(),
        pieces: PieceList::from_pieces(["test", "data"]),
        mode: Mode::Single {
          length: Bytes(2 * 16 * 1024),
          md5sum: None,
        },
        update_url: None,
      },
    };
    env.write("test.torrent", metainfo.serialize().unwrap());
    assert_matches!(env.run(), Err(Error::MetainfoMissingTrackers));
  }
}
