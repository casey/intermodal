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
    long = "open",
    short = "O",
    help = "Open generated magnet link. Uses `xdg-open`, `gnome-open`, or `kde-open` on Linux; \
            `open` on macOS; and `cmd /C start` on Windows"
  )]
  open: bool,
  #[structopt(
    long = "peer",
    short = "p",
    value_name = "PEER",
    help = "Add `PEER` to magnet link."
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

  #[test]
  fn open() {
    let mut create_env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
      ],
      tree: {
        foo: "",
      },
    };

    assert_matches!(create_env.run(), Ok(()));

    let torrent = create_env.resolve("foo.torrent");

    let mut env = test_env! {
      args: [
        "torrent",
        "link",
        "--input",
        &torrent,
        "--open",
      ],
      tree: {},
    };

    let opened = env.resolve("opened.txt");

    let link = "magnet:?xt=urn:btih:516735f4b80f2b5487eed5f226075bdcde33a54e&dn=foo";

    let expected = if cfg!(target_os = "windows") {
      let script = env.resolve("open.bat");
      fs::write(&script, format!("echo \"%3\" > {}", opened.display())).unwrap();
      format!("{} \r\n", link)
    } else {
      let script = env.resolve(&Platform::opener().unwrap()[0]);
      fs::write(
        &script,
        format!("#!/usr/bin/env sh\necho $1 > {}", opened.display()),
      )
      .unwrap();

      Command::new("chmod")
        .arg("+x")
        .arg(&script)
        .status()
        .unwrap();

      format!("{}\n", link)
    };

    const KEY: &str = "PATH";
    let path = env::var_os(KEY).unwrap();
    let mut split = env::split_paths(&path)
      .into_iter()
      .collect::<Vec<PathBuf>>();
    split.insert(0, env.dir().to_owned());
    let new = env::join_paths(split).unwrap();
    env::set_var(KEY, new);

    assert_matches!(env.run(), Ok(()));

    let start = Instant::now();

    while start.elapsed() < Duration::new(2, 0) {
      if let Ok(text) = fs::read_to_string(&opened) {
        assert_eq!(text, expected);
        return;
      }
    }

    panic!("Failed to read `opened.txt`.");
  }
}
