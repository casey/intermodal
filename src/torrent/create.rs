use crate::common::*;

#[derive(StructOpt)]
pub(crate) struct Create {
  #[structopt(name = "ANNOUNCE", long = "announce", required(true))]
  announce: Vec<String>,
  #[structopt(name = "COMMENT", long = "comment")]
  comment: Option<String>,
  #[structopt(name = "INPUT", long = "input")]
  input: PathBuf,
  #[structopt(name = "MD5SUM", long = "md5sum")]
  md5sum: bool,
  #[structopt(name = "NAME", long = "name")]
  name: Option<String>,
  #[structopt(name = "NO-CREATED-BY", long = "no-created-by")]
  no_created_by: bool,
  #[structopt(name = "NO-CREATION-DATE", long = "no-creation-date")]
  no_creation_date: bool,
  #[structopt(name = "OUTPUT", long = "output")]
  output: Option<PathBuf>,
  #[structopt(name = "PIECE-LENGTH", long = "piece-length", default_value = "524288")]
  piece_length: u32,
  #[structopt(name = "PRIVATE", long = "private")]
  private: bool,
}

impl Create {
  pub(crate) fn run(self, env: &Env) -> Result<(), Error> {
    let input = env.resolve(&self.input);

    let mut announce_list = Vec::new();
    for announce in &self.announce {
      let tier = announce
        .split(',')
        .map(str::to_string)
        .collect::<Vec<String>>();

      tier
        .iter()
        .map(|announce| announce.parse())
        .collect::<Result<Vec<Url>, url::ParseError>>()
        .context(error::AnnounceUrlParse)?;

      announce_list.push(tier);
    }

    let announce = if let Some(primary) = announce_list.first().and_then(|tier| tier.first()) {
      primary.clone()
    } else {
      return Err(Error::AnnounceEmpty);
    };

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

    let (mode, pieces) = Hasher::hash(&input, self.md5sum, self.piece_length)?;

    let info = Info {
      piece_length: self.piece_length,
      mode,
      pieces,
      name,
      private,
    };

    let metainfo = Metainfo {
      comment: self.comment,
      encoding: consts::ENCODING_UTF8.to_string(),
      announce,
      announce_list,
      creation_date,
      created_by,
      info,
    };

    let bytes = serde_bencode::ser::to_bytes(&metainfo)?;

    fs::write(&output, bytes).context(error::Filesystem { path: &output })?;

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
    assert_matches!(env.run(), Err(Error::AnnounceUrlParse { .. }));
  }

  #[test]
  fn announce_single() {
    let mut env = environment(&["--input", "foo", "--announce", "http://bar"]);
    fs::write(env.resolve("foo"), "").unwrap();
    env.run().unwrap();
    let torrent = env.resolve("foo.torrent");
    let bytes = fs::read(torrent).unwrap();
    let metainfo = serde_bencode::de::from_bytes::<Metainfo>(&bytes).unwrap();
    assert_eq!(metainfo.announce, "http://bar");
    assert_eq!(metainfo.announce_list, vec![vec!["http://bar"]]);
  }

  #[test]
  fn announce_single_tier() {
    let mut env = environment(&["--input", "foo", "--announce", "http://bar,http://baz"]);
    fs::write(env.resolve("foo"), "").unwrap();
    env.run().unwrap();
    let torrent = env.resolve("foo.torrent");
    let bytes = fs::read(torrent).unwrap();
    let metainfo = serde_bencode::de::from_bytes::<Metainfo>(&bytes).unwrap();
    assert_eq!(metainfo.announce, "http://bar");
    assert_eq!(
      metainfo.announce_list,
      vec![vec!["http://bar", "http://baz"]]
    );
  }

  #[test]
  fn announce_multiple_tiers() {
    let mut env = environment(&[
      "--input",
      "foo",
      "--announce",
      "http://bar,http://baz",
      "--announce",
      "http://abc,http://xyz",
    ]);
    fs::write(env.resolve("foo"), "").unwrap();
    env.run().unwrap();
    let torrent = env.resolve("foo.torrent");
    let bytes = fs::read(torrent).unwrap();
    let metainfo = serde_bencode::de::from_bytes::<Metainfo>(&bytes).unwrap();
    assert_eq!(metainfo.announce, "http://bar");
    assert_eq!(
      metainfo.announce_list,
      vec![
        vec!["http://bar", "http://baz"],
        vec!["http://abc", "http://xyz"],
      ]
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
}
