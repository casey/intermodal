use crate::common::*;
use rayon::prelude::*;

const URI_HELP: &str = "Generate a torrent file from a magnet URI";

const INPUT_FLAG: &str = "input-flag";
const INPUT_POSITIONAL: &str = "<INPUT>";
const INPUT_HELP: &str = "The magnet URI.";

#[derive(StructOpt)]
#[structopt(
  help_message(consts::HELP_MESSAGE),
  version_message(consts::VERSION_MESSAGE),
  about(URI_HELP)
)]
pub(crate) struct FromLink {
  #[structopt(
    name = INPUT_FLAG,
    long = "input",
    short = "i",
    value_name = "INPUT",
    empty_values = false,
    help = INPUT_HELP,
  )]
  input_flag: Option<MagnetLink>,
  #[structopt(
    name = INPUT_POSITIONAL,
    value_name = "INPUT",
    empty_values = false,
    required_unless = INPUT_FLAG,
    conflicts_with = INPUT_FLAG,
    help = INPUT_HELP,
  )]
  input_positional: Option<MagnetLink>,
  #[structopt(
    long = "output",
    short = "o",
    value_name = "TARGET",
    empty_values(false),
    required_if(INPUT_FLAG, "-"),
    required_if(INPUT_POSITIONAL, "-"),
    help = "Save `.torrent` file to `TARGET`; if omitted, the parameter is set to `./${INFOHASH}.torrent`."
  )]
  output: Option<PathBuf>,
}

impl FromLink {
  pub(crate) fn run(self, env: &mut Env, options: &Options) -> Result<()> {
    let link = xor_args(
      "input_flag",
      &self.input_flag,
      "input_positional",
      &self.input_positional,
    )?;

    let infohash = link.infohash;

    if !options.quiet {
      errln!(env, "Sending announce to all trackers.")?;
    }

    let (tx, rx) = channel();
    link.trackers.par_iter().for_each_with(tx, |s, x| {
      let Ok(c) = tracker::Client::from_url(x) else {
        return;
      };
      if let Ok(list) = c.announce_exchange(&infohash) {
        for p in list {
          s.send(p).ok();
        }
      }
    });

    let peers: HashSet<_> = rx.iter().collect();

    if !options.quiet {
      errln!(env, "Trackers returned {} peers.", peers.len())?;
    }

    let info = peers.par_iter().find_map_any(|addr| {
      peer::Client::connect(addr, infohash)
        .ok()
        .and_then(|c| c.fetch_info_dict().ok())
    });

    let metainfo = match info {
      Some(info) => Metainfo {
        announce: None,
        announce_list: Some(vec![link.trackers.iter().map(Url::to_string).collect()]),
        nodes: None,
        comment: None,
        created_by: None,
        creation_date: None,
        encoding: None,
        info,
      },
      None => return Err(Error::FromLinkNoInfo),
    };

    if !options.quiet {
      errln!(env, "Received info dict.")?;
    }

    let mut path = self.output.unwrap_or_else(|| {
      let mut path = PathBuf::new();
      path.push(infohash.to_string());
      path.set_extension("torrent");
      path
    });
    path = env.resolve(path)?;

    fs::File::create(&path)
      .context(error::Filesystem { path: path.clone() })
      .and_then(|mut f| {
        f.write_all(&metainfo.serialize()?)
          .context(error::Filesystem { path: path.clone() })
      })?;

    if !options.quiet {
      errln!(env, "Torrent file written to `{}`.", path.display())?;
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn input_required() {
    test_env! {
      args: [
        "torrent",
        "from-link",
      ],
      tree: {
      },
      matches: Err(Error::Clap { .. }),
    };
  }

  #[test]
  #[ignore]
  fn test_no_info() {
    let tracker_url = "udp://1.2.3.4:1337";
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
    let link = MagnetLink::from_metainfo_lossy(&metainfo).unwrap();
    let mut env = test_env! {
      args: [
        "torrent",
        "from-link",
        link.to_url().as_str(),
      ],
      tree: {},
    };
    assert_matches!(env.run(), Err(Error::FromLinkNoInfo));
  }

  #[test]
  fn with_one_good_seeder() {
    let info = Info {
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
    };
    let infohash = info.infohash_lossy().unwrap();
    let (_, addr_s) = peer::Client::spawn_info_dict_seeder(&info);
    let records = HashMap::from([(infohash.into(), HashSet::from([addr_s]))]);
    let (_, addr_d) = tracker::Daemon::spawn_with_records(records);
    let tracker_url = addr_d.to_string();

    let metainfo = Metainfo {
      announce: None,
      announce_list: Some(vec![vec![format!("udp://{}", tracker_url)]]),
      nodes: None,
      comment: None,
      created_by: None,
      creation_date: None,
      encoding: None,
      info,
    };
    let link = MagnetLink::from_metainfo_lossy(&metainfo)
      .unwrap()
      .to_url()
      .to_string();

    let mut env = test_env! {
      args: [
        "torrent",
        "from-link",
        link,
      ],
      tree: {},
    };
    env.assert_ok();
    assert_eq!(metainfo, env.load_metainfo(format!("{infohash}.torrent")));
  }

  #[test]
  fn with_one_good_seeder_and_output_flag() {
    let info = Info {
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
    };
    let infohash = info.infohash_lossy().unwrap();
    let (_, addr_s) = peer::Client::spawn_info_dict_seeder(&info);
    let records = HashMap::from([(infohash.into(), HashSet::from([addr_s]))]);
    let (_, addr_d) = tracker::Daemon::spawn_with_records(records);
    let tracker_url = addr_d.to_string();

    let metainfo = Metainfo {
      announce: None,
      announce_list: Some(vec![vec![format!("udp://{}", tracker_url)]]),
      nodes: None,
      comment: None,
      created_by: None,
      creation_date: None,
      encoding: None,
      info,
    };
    let link = MagnetLink::from_metainfo_lossy(&metainfo)
      .unwrap()
      .to_url()
      .to_string();

    let mut env = test_env! {
      args: [
        "torrent",
        "from-link",
        link,
        "-o",
        "foo.torrent",
      ],
      tree: {},
    };
    env.assert_ok();
    assert_eq!(metainfo, env.load_metainfo("foo.torrent"));
  }

  #[test]
  fn with_one_good_seeder_many_bad_seeders() {
    let info = Info {
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
    };
    let (_, addr_s) = peer::Client::spawn_info_dict_seeder(&info);
    let mut set = HashSet::from([addr_s]);
    for p in [1337, 12345, 54321] {
      set.insert((Ipv4Addr::LOCALHOST, p).into());
    }
    let infohash = info.infohash_lossy().unwrap();
    let records = HashMap::from([(infohash.into(), set)]);
    let (_, addr_d) = tracker::Daemon::spawn_with_records(records);
    let tracker_url = addr_d.to_string();
    let metainfo = Metainfo {
      announce: None,
      announce_list: Some(vec![vec![format!("udp://{}", tracker_url)]]),
      nodes: None,
      comment: None,
      created_by: None,
      creation_date: None,
      encoding: None,
      info,
    };
    let link = MagnetLink::from_metainfo_lossy(&metainfo)
      .unwrap()
      .to_url()
      .to_string();

    let mut env = test_env! {
      args: [
        "torrent",
        "from-link",
        link,
        "-o",
        "foo.torrent",
      ],
      tree: {},
    };
    env.assert_ok();
    assert_eq!(metainfo, env.load_metainfo("foo.torrent"));
  }
}
