use crate::common::*;

#[derive(StructOpt)]
#[structopt(
  help_message(consts::HELP_MESSAGE),
  version_message(consts::VERSION_MESSAGE),
  about("Create a `.torrent` file.")
)]
pub(crate) struct Create {
  #[structopt(
    name = "ANNOUNCE",
    long = "announce",
    required(true),
    help = "Use `ANNOUNCE` as the primary tracker announce URL.",
    long_help = "Use `ANNOUNCE` as the primary tracker announce URL. To supply multiple announce URLs, also use `--announce-tier`."
  )]
  announce: Url,
  #[structopt(
    long = "announce-tier",
    name = "ANNOUNCE-TIER",
    help = "Add `ANNOUNCE-TIER` to list of tracker announce tiers.",
    long_help = "\
Add `ANNOUNCE-TIER` to list of tracker announce tiers. Each instance adds a new tier. To add multiple trackers to a given tier, separate their announce URLs with commas: 

`--announce-tier udp://example.com:80/announce,https://example.net:443/announce`

Announce tiers are stored in the `announce-list` key of the top-level metainfo dictionary as a list of lists of strings, as defined by BEP 12: Multitracker Metadata Extension.

Note: Many BitTorrent clients do not implement the behavior described in BEP 12. See the discussion here for more details: https://github.com/bittorrent/bittorrent.org/issues/82"
  )]
  announce_tiers: Vec<String>,
  #[structopt(
    name = "COMMENT",
    long = "comment",
    help = "Include `COMMENT` in generated `.torrent` file.",
    long_help = "Include `COMMENT` in generated `.torrent` file. Stored under `comment` key of top-level metainfo dictionary."
  )]
  comment: Option<String>,
  #[structopt(
    name = "INPUT",
    long = "input",
    help = "Read torrent contents from `INPUT`.",
    long_help = "Read torrent contents from `INPUT`. If `INPUT` is a file, torrent will be a single-file torrent, otherwise if `INPUT` is a directory, torrent will be a multi-file torrent."
  )]
  input: PathBuf,
  #[structopt(
    name = "MD5SUM",
    long = "md5sum",
    help = "Include MD5 checksum of each file in the torrent. N.B. MD5 is cryptographically broken and only suitable for safeguarding against accidental corruption.",
    long_help = "Include MD5 checksum of each file in the torrent. N.B. MD5 is cryptographically broken and only suitable for checking for accidental corruption."
  )]
  md5sum: bool,
  #[structopt(
    name = "NAME",
    long = "name",
    help = "Set name of torrent to `NAME`. Defaults to the filename of `--input`."
  )]
  name: Option<String>,
  #[structopt(
    name = "NO-CREATED-BY",
    long = "no-created-by",
    help = "Do not populate `created by` key of generated torrent with imdl version information."
  )]
  no_created_by: bool,
  #[structopt(
    name = "NO-CREATION-DATE",
    long = "no-creation-date",
    help = "Do not populate `creation date` key of generated torrent with current time."
  )]
  no_creation_date: bool,
  #[structopt(
    name = "OPEN",
    long = "open",
    help = "Open `.torrent` file after creation",
    long_help = "Open `.torrent` file after creation. Uses `xdg-open`, `gnome-open`, or `kde-open` on Linux; `open` on macOS; and `cmd /C start on Windows"
  )]
  open: bool,
  #[structopt(
    name = "OUTPUT",
    long = "output",
    help = "Save `.torrent` file to `OUTPUT`. Defaults to `$INPUT.torrent`."
  )]
  output: Option<PathBuf>,
  #[structopt(
    name = "PIECE-LENGTH",
    long = "piece-length",
    default_value = "512KiB",
    help = "Set piece length to `PIECE-LENGTH` bytes.",
    long_help = "Set piece length to `PIECE-LENGTH` bytes. Accepts SI units, e.g. kib, mib, and gib."
  )]
  piece_length: Bytes,
  #[structopt(
    name = "PRIVATE",
    long = "private",
    help = "Set the `private` flag.",
    long_help = "Set the `private` flag. Torrent clients that understand the flag and participate in the swarm of a torrent with the flag set will only announce themselves to the announce URLs included in the torrent, and will not use other peer discovery mechanisms, such as the DHT or local peer discovery. See BEP 27: Private Torrents for more information."
  )]
  private: bool,
}

impl Create {
  pub(crate) fn run(self, env: &Env) -> Result<(), Error> {
    let piece_length: u32 = self
      .piece_length
      .0
      .try_into()
      .map_err(|_| Error::PieceLength {
        bytes: self.piece_length,
      })?;

    let input = env.resolve(&self.input);

    let mut announce_list = Vec::new();
    for tier in &self.announce_tiers {
      let tier = tier.split(',').map(str::to_string).collect::<Vec<String>>();

      tier
        .iter()
        .map(|announce| announce.parse())
        .collect::<Result<Vec<Url>, url::ParseError>>()
        .context(error::AnnounceUrlParse)?;

      announce_list.push(tier);
    }

    let filename = input.file_name().ok_or_else(|| Error::FilenameExtract {
      path: input.clone(),
    })?;

    let name = match &self.name {
      Some(name) => name.clone(),
      None => filename
        .to_str()
        .ok_or_else(|| Error::FilenameDecode {
          filename: filename.to_os_string(),
        })?
        .to_owned(),
    };

    let output = self
      .output
      .as_ref()
      .map(|output| env.resolve(&output))
      .unwrap_or_else(|| {
        let mut torrent_name = name.to_owned();
        torrent_name.push_str(".torrent");

        input.parent().unwrap().join(torrent_name)
      });

    let private = if self.private { 1 } else { 0 };

    let creation_date = if self.no_creation_date {
      None
    } else {
      Some(
        SystemTime::now()
          .duration_since(SystemTime::UNIX_EPOCH)?
          .as_secs(),
      )
    };

    let created_by = if self.no_created_by {
      None
    } else {
      Some(String::from(consts::CREATED_BY_DEFAULT))
    };

    let (mode, pieces) = Hasher::hash(&input, self.md5sum, piece_length)?;

    let info = Info {
      piece_length,
      mode,
      pieces,
      name,
      private,
    };

    let metainfo = Metainfo {
      comment: self.comment,
      encoding: consts::ENCODING_UTF8.to_string(),
      announce: self.announce.to_string(),
      announce_list: if announce_list.is_empty() {
        None
      } else {
        Some(announce_list)
      },
      creation_date,
      created_by,
      info,
    };

    let bytes = serde_bencode::ser::to_bytes(&metainfo)?;

    fs::write(&output, bytes).context(error::Filesystem { path: &output })?;

    if self.open {
      Platform::open(&output)?;
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use crate::test_env::TestEnv;

  fn environment(args: &[&str]) -> TestEnv {
    testing::env(["torrent", "create"].iter().chain(args).cloned())
  }

  #[test]
  fn require_input_argument() {
    let mut env = environment(&[]);
    assert!(matches!(env.run(), Err(Error::Clap { .. })));
  }

  #[test]
  fn require_input_present() {
    let mut env = environment(&["--input", "foo", "--announce", "http://bar"]);
    assert!(matches!(env.run(), Err(Error::Filesystem { .. })));
  }

  #[test]
  fn torrent_file_is_bencode_dict() {
    let mut env = environment(&["--input", "foo", "--announce", "http://bar"]);
    fs::write(env.resolve("foo"), "").unwrap();
    env.run().unwrap();
    let torrent = env.resolve("foo.torrent");
    let bytes = fs::read(torrent).unwrap();
    let value = bencode::Value::decode(&bytes).unwrap();
    assert!(matches!(value, bencode::Value::Dict(_)));
  }

  #[test]
  fn privacy_defaults_to_false() {
    let mut env = environment(&["--input", "foo", "--announce", "http://bar"]);
    fs::write(env.resolve("foo"), "").unwrap();
    env.run().unwrap();
    let torrent = env.resolve("foo.torrent");
    let bytes = fs::read(torrent).unwrap();
    let metainfo = serde_bencode::de::from_bytes::<Metainfo>(&bytes).unwrap();
    assert_eq!(metainfo.info.private, 0);
  }

  #[test]
  fn privacy_flag_sets_privacy() {
    let mut env = environment(&["--input", "foo", "--announce", "http://bar", "--private"]);
    fs::write(env.resolve("foo"), "").unwrap();
    env.run().unwrap();
    let torrent = env.resolve("foo.torrent");
    let bytes = fs::read(torrent).unwrap();
    let metainfo = serde_bencode::de::from_bytes::<Metainfo>(&bytes).unwrap();
    assert_eq!(metainfo.info.private, 1);
  }

  #[test]
  fn tracker_flag_must_be_url() {
    let mut env = environment(&["--input", "foo", "--announce", "bar"]);
    fs::write(env.resolve("foo"), "").unwrap();
    assert_matches!(env.run(), Err(Error::Clap { .. }));
  }

  #[test]
  fn announce_single() {
    let mut env = environment(&["--input", "foo", "--announce", "http://bar"]);
    fs::write(env.resolve("foo"), "").unwrap();
    env.run().unwrap();
    let torrent = env.resolve("foo.torrent");
    let bytes = fs::read(torrent).unwrap();
    let metainfo = serde_bencode::de::from_bytes::<Metainfo>(&bytes).unwrap();
    assert_eq!(metainfo.announce, "http://bar/");
    assert!(metainfo.announce_list.is_none());
  }

  #[test]
  fn announce_udp() {
    let mut env = environment(&[
      "--input",
      "foo",
      "--announce",
      "udp://tracker.opentrackr.org:1337/announce",
    ]);
    fs::write(env.resolve("foo"), "").unwrap();
    env.run().unwrap();
    let torrent = env.resolve("foo.torrent");
    let bytes = fs::read(torrent).unwrap();
    let metainfo = serde_bencode::de::from_bytes::<Metainfo>(&bytes).unwrap();
    assert_eq!(
      metainfo.announce,
      "udp://tracker.opentrackr.org:1337/announce"
    );
    assert!(metainfo.announce_list.is_none());
  }

  #[test]
  fn announce_wss_tracker() {
    let mut env = environment(&["--input", "foo", "--announce", "wss://tracker.btorrent.xyz"]);
    fs::write(env.resolve("foo"), "").unwrap();
    env.run().unwrap();
    let torrent = env.resolve("foo.torrent");
    let bytes = fs::read(torrent).unwrap();
    let metainfo = serde_bencode::de::from_bytes::<Metainfo>(&bytes).unwrap();
    assert_eq!(metainfo.announce, "wss://tracker.btorrent.xyz/");
    assert!(metainfo.announce_list.is_none());
  }

  #[test]
  fn announce_single_tier() {
    let mut env = environment(&[
      "--input",
      "foo",
      "--announce",
      "http://bar",
      "--announce-tier",
      "http://bar,http://baz",
    ]);
    fs::write(env.resolve("foo"), "").unwrap();
    env.run().unwrap();
    let torrent = env.resolve("foo.torrent");
    let bytes = fs::read(torrent).unwrap();
    let metainfo = serde_bencode::de::from_bytes::<Metainfo>(&bytes).unwrap();
    assert_eq!(metainfo.announce, "http://bar/");
    assert_eq!(
      metainfo.announce_list,
      Some(vec![vec!["http://bar".into(), "http://baz".into()]]),
    );
  }

  #[test]
  fn announce_multiple_tiers() {
    let mut env = environment(&[
      "--input",
      "foo",
      "--announce",
      "http://bar",
      "--announce-tier",
      "http://bar,http://baz",
      "--announce-tier",
      "http://abc,http://xyz",
    ]);
    fs::write(env.resolve("foo"), "").unwrap();
    env.run().unwrap();
    let torrent = env.resolve("foo.torrent");
    let bytes = fs::read(torrent).unwrap();
    let metainfo = serde_bencode::de::from_bytes::<Metainfo>(&bytes).unwrap();
    assert_eq!(metainfo.announce, "http://bar/");
    assert_eq!(
      metainfo.announce_list,
      Some(vec![
        vec!["http://bar".into(), "http://baz".into()],
        vec!["http://abc".into(), "http://xyz".into()],
      ])
    );
  }

  #[test]
  fn comment_default() {
    let mut env = environment(&["--input", "foo", "--announce", "http://bar"]);
    fs::write(env.resolve("foo"), "").unwrap();
    env.run().unwrap();
    let torrent = env.resolve("foo.torrent");
    let bytes = fs::read(torrent).unwrap();
    let metainfo = serde_bencode::de::from_bytes::<Metainfo>(&bytes).unwrap();
    assert_eq!(metainfo.comment, None);
  }

  #[test]
  fn comment_set() {
    let mut env = environment(&[
      "--input",
      "foo",
      "--announce",
      "http://bar",
      "--comment",
      "Hello, world!",
    ]);
    fs::write(env.resolve("foo"), "").unwrap();
    env.run().unwrap();
    let torrent = env.resolve("foo.torrent");
    let bytes = fs::read(torrent).unwrap();
    let metainfo = serde_bencode::de::from_bytes::<Metainfo>(&bytes).unwrap();
    assert_eq!(metainfo.comment.unwrap(), "Hello, world!");
  }

  #[test]
  fn piece_length_default() {
    let mut env = environment(&["--input", "foo", "--announce", "http://bar"]);
    fs::write(env.resolve("foo"), "").unwrap();
    env.run().unwrap();
    let torrent = env.resolve("foo.torrent");
    let bytes = fs::read(torrent).unwrap();
    let metainfo = serde_bencode::de::from_bytes::<Metainfo>(&bytes).unwrap();
    assert_eq!(metainfo.info.piece_length, 512 * 2u32.pow(10));
  }

  #[test]
  fn piece_length_override() {
    let mut env = environment(&[
      "--input",
      "foo",
      "--announce",
      "http://bar",
      "--piece-length",
      "1",
    ]);
    fs::write(env.resolve("foo"), "").unwrap();
    env.run().unwrap();
    let torrent = env.resolve("foo.torrent");
    let bytes = fs::read(torrent).unwrap();
    let metainfo = serde_bencode::de::from_bytes::<Metainfo>(&bytes).unwrap();
    assert_eq!(metainfo.info.piece_length, 1);
  }

  #[test]
  fn si_piece_size() {
    let mut env = environment(&[
      "--input",
      "foo",
      "--announce",
      "http://bar",
      "--piece-length",
      "0.5MiB",
    ]);
    fs::write(env.resolve("foo"), "").unwrap();
    env.run().unwrap();
    let torrent = env.resolve("foo.torrent");
    let bytes = fs::read(torrent).unwrap();
    let metainfo = serde_bencode::de::from_bytes::<Metainfo>(&bytes).unwrap();
    assert_eq!(metainfo.info.piece_length, 512 * 1024);
  }

  #[test]
  fn name() {
    let mut env = environment(&[
      "--input",
      "foo",
      "--announce",
      "http://bar",
      "--piece-length",
      "1",
    ]);
    fs::write(env.resolve("foo"), "").unwrap();
    env.run().unwrap();
    let torrent = env.resolve("foo.torrent");
    let bytes = fs::read(torrent).unwrap();
    let metainfo = serde_bencode::de::from_bytes::<Metainfo>(&bytes).unwrap();
    assert_eq!(metainfo.info.name, "foo");
  }

  #[test]
  fn name_subdir() {
    let mut env = environment(&[
      "--input",
      "foo/bar",
      "--announce",
      "http://bar",
      "--piece-length",
      "1",
    ]);
    let dir = env.resolve("foo");
    fs::create_dir(&dir).unwrap();
    fs::write(dir.join("bar"), "").unwrap();
    env.run().unwrap();
    let torrent = dir.join("bar.torrent");
    let bytes = fs::read(torrent).unwrap();
    let metainfo = serde_bencode::de::from_bytes::<Metainfo>(&bytes).unwrap();
    assert_eq!(metainfo.info.name, "bar");
  }

  #[test]
  fn destination_override() {
    let mut env = environment(&[
      "--input",
      "foo",
      "--output",
      "x.torrent",
      "--announce",
      "http://bar",
    ]);
    fs::write(env.resolve("foo"), "").unwrap();
    env.run().unwrap();
    let torrent = env.resolve("x.torrent");
    let bytes = fs::read(torrent).unwrap();
    serde_bencode::de::from_bytes::<Metainfo>(&bytes).unwrap();
  }

  #[test]
  fn created_by_default() {
    let mut env = environment(&["--input", "foo", "--announce", "http://bar"]);
    fs::write(env.resolve("foo"), "").unwrap();
    env.run().unwrap();
    let torrent = env.resolve("foo.torrent");
    let bytes = fs::read(torrent).unwrap();
    let metainfo = serde_bencode::de::from_bytes::<Metainfo>(&bytes).unwrap();
    assert_eq!(metainfo.created_by.unwrap(), consts::CREATED_BY_DEFAULT);
  }

  #[test]
  fn created_by_unset() {
    let mut env = environment(&[
      "--input",
      "foo",
      "--announce",
      "http://bar",
      "--no-created-by",
    ]);
    fs::write(env.resolve("foo"), "").unwrap();
    env.run().unwrap();
    let torrent = env.resolve("foo.torrent");
    let bytes = fs::read(torrent).unwrap();
    let metainfo = serde_bencode::de::from_bytes::<Metainfo>(&bytes).unwrap();
    assert_eq!(metainfo.created_by, None);
  }

  #[test]
  fn encoding() {
    let mut env = environment(&["--input", "foo", "--announce", "http://bar"]);
    fs::write(env.resolve("foo"), "").unwrap();
    env.run().unwrap();
    let torrent = env.resolve("foo.torrent");
    let bytes = fs::read(torrent).unwrap();
    let metainfo = serde_bencode::de::from_bytes::<Metainfo>(&bytes).unwrap();
    assert_eq!(metainfo.encoding, "UTF-8");
  }

  #[test]
  fn created_date_default() {
    let mut env = environment(&["--input", "foo", "--announce", "http://bar"]);
    fs::write(env.resolve("foo"), "").unwrap();
    let now = SystemTime::now()
      .duration_since(SystemTime::UNIX_EPOCH)
      .unwrap()
      .as_secs();
    env.run().unwrap();
    let torrent = env.resolve("foo.torrent");
    let bytes = fs::read(torrent).unwrap();
    let metainfo = serde_bencode::de::from_bytes::<Metainfo>(&bytes).unwrap();
    assert!(metainfo.creation_date.unwrap() < now + 10);
    assert!(metainfo.creation_date.unwrap() > now - 10);
  }

  #[test]
  fn created_date_unset() {
    let mut env = environment(&[
      "--input",
      "foo",
      "--announce",
      "http://bar",
      "--no-creation-date",
    ]);
    fs::write(env.resolve("foo"), "").unwrap();
    env.run().unwrap();
    let torrent = env.resolve("foo.torrent");
    let bytes = fs::read(torrent).unwrap();
    let metainfo = serde_bencode::de::from_bytes::<Metainfo>(&bytes).unwrap();
    assert_eq!(metainfo.creation_date, None);
  }

  #[test]
  fn single_small() {
    let mut env = environment(&["--input", "foo", "--announce", "http://bar"]);
    let contents = "bar";
    fs::write(env.resolve("foo"), contents).unwrap();
    env.run().unwrap();
    let torrent = env.resolve("foo.torrent");
    let bytes = fs::read(torrent).unwrap();
    let metainfo = serde_bencode::de::from_bytes::<Metainfo>(&bytes).unwrap();
    assert_eq!(metainfo.info.pieces, Sha1::from(contents).digest().bytes());
    assert_eq!(
      metainfo.info.mode,
      Mode::Single {
        length: contents.len() as u64,
        md5sum: None,
      }
    )
  }

  #[test]
  fn single_one_byte_piece() {
    let mut env = environment(&[
      "--input",
      "foo",
      "--announce",
      "http://bar",
      "--piece-length",
      "1",
    ]);
    let contents = "bar";
    fs::write(env.resolve("foo"), contents).unwrap();
    env.run().unwrap();
    let torrent = env.resolve("foo.torrent");
    let bytes = fs::read(torrent).unwrap();
    let metainfo = serde_bencode::de::from_bytes::<Metainfo>(&bytes).unwrap();
    let pieces = Sha1::from("b")
      .digest()
      .bytes()
      .iter()
      .chain(Sha1::from("a").digest().bytes().iter())
      .chain(Sha1::from("r").digest().bytes().iter())
      .cloned()
      .collect::<Vec<u8>>();

    assert_eq!(metainfo.info.pieces, pieces);
    assert_eq!(
      metainfo.info.mode,
      Mode::Single {
        length: contents.len() as u64,
        md5sum: None,
      }
    )
  }

  #[test]
  fn single_empty() {
    let mut env = environment(&["--input", "foo", "--announce", "http://bar"]);
    let contents = "";
    fs::write(env.resolve("foo"), contents).unwrap();
    env.run().unwrap();
    let torrent = env.resolve("foo.torrent");
    let bytes = fs::read(torrent).unwrap();
    let metainfo = serde_bencode::de::from_bytes::<Metainfo>(&bytes).unwrap();
    assert_eq!(metainfo.info.pieces.len(), 0);
    assert_eq!(
      metainfo.info.mode,
      Mode::Single {
        length: 0,
        md5sum: None,
      }
    )
  }

  #[test]
  fn multiple_no_files() {
    let mut env = environment(&["--input", "foo", "--announce", "http://bar"]);
    let dir = env.resolve("foo");
    fs::create_dir(&dir).unwrap();
    env.run().unwrap();
    let torrent = env.resolve("foo.torrent");
    let bytes = fs::read(torrent).unwrap();
    let metainfo = serde_bencode::de::from_bytes::<Metainfo>(&bytes).unwrap();
    assert_eq!(metainfo.info.pieces.len(), 0);
    assert_eq!(metainfo.info.mode, Mode::Multiple { files: Vec::new() })
  }

  #[test]
  fn multiple_one_file() {
    let mut env = environment(&["--input", "foo", "--announce", "http://bar"]);
    let dir = env.resolve("foo");
    fs::create_dir(&dir).unwrap();
    let file = dir.join("bar");
    let contents = "bar";
    fs::write(file, contents).unwrap();
    env.run().unwrap();
    let torrent = env.resolve("foo.torrent");
    let bytes = fs::read(torrent).unwrap();
    let metainfo = serde_bencode::de::from_bytes::<Metainfo>(&bytes).unwrap();
    assert_eq!(metainfo.info.pieces, Sha1::from(contents).digest().bytes());
    assert_eq!(metainfo.info.mode, Mode::Multiple { files: Vec::new() })
  }

  #[test]
  fn multiple_three_files() {
    let mut env = environment(&["--input", "foo", "--announce", "http://bar"]);
    let dir = env.resolve("foo");
    fs::create_dir(&dir).unwrap();
    fs::write(dir.join("a"), "abc").unwrap();
    fs::write(dir.join("x"), "xyz").unwrap();
    fs::write(dir.join("h"), "hij").unwrap();
    env.run().unwrap();
    let torrent = env.resolve("foo.torrent");
    let bytes = fs::read(torrent).unwrap();
    let metainfo = serde_bencode::de::from_bytes::<Metainfo>(&bytes).unwrap();
    assert_eq!(
      metainfo.info.pieces,
      Sha1::from("abchijxyz").digest().bytes()
    );
    assert_eq!(metainfo.info.mode, Mode::Multiple { files: Vec::new() })
  }

  #[test]
  fn open() {
    let mut env = environment(&["--input", "foo", "--announce", "http://bar", "--open"]);

    let opened = env.resolve("opened.txt");
    let torrent = env.resolve("foo.torrent");

    let expected = if cfg!(target_os = "windows") {
      let script = env.resolve("open.bat");
      fs::write(&script, format!("echo %3 > {}", opened.display())).unwrap();
      format!("{} \r\n", torrent.display())
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

      format!("{}\n", torrent.display())
    };

    const KEY: &str = "PATH";
    let path = env::var_os(KEY).unwrap();
    let mut split = env::split_paths(&path)
      .into_iter()
      .collect::<Vec<PathBuf>>();
    split.insert(0, env.dir().to_owned());
    let new = env::join_paths(split).unwrap();
    env::set_var(KEY, new);

    fs::write(env.resolve("foo"), "").unwrap();
    env.run().unwrap();

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
