use crate::common::*;

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub(crate) struct Metainfo {
  #[serde(
    skip_serializing_if = "Option::is_none",
    default,
    with = "unwrap_or_skip"
  )]
  pub(crate) announce: Option<String>,
  #[serde(
    rename = "announce-list",
    skip_serializing_if = "Option::is_none",
    default,
    with = "unwrap_or_skip"
  )]
  pub(crate) announce_list: Option<Vec<Vec<String>>>,
  #[serde(
    skip_serializing_if = "Option::is_none",
    default,
    with = "unwrap_or_skip"
  )]
  pub(crate) comment: Option<String>,
  #[serde(
    rename = "created by",
    skip_serializing_if = "Option::is_none",
    default,
    with = "unwrap_or_skip"
  )]
  pub(crate) created_by: Option<String>,
  #[serde(
    rename = "creation date",
    skip_serializing_if = "Option::is_none",
    default,
    with = "unwrap_or_skip"
  )]
  pub(crate) creation_date: Option<u64>,
  #[serde(
    skip_serializing_if = "Option::is_none",
    default,
    with = "unwrap_or_skip"
  )]
  pub(crate) encoding: Option<String>,
  pub(crate) info: Info,
  #[serde(
    skip_serializing_if = "Option::is_none",
    default,
    with = "unwrap_or_skip"
  )]
  pub(crate) nodes: Option<Vec<HostPort>>,
}

impl Metainfo {
  pub(crate) fn from_input(input: &Input) -> Result<Metainfo> {
    Self::deserialize(&input.source, &input.data)
  }

  pub(crate) fn deserialize(source: &InputTarget, data: &[u8]) -> Result<Metainfo, Error> {
    let metainfo = bendy::serde::de::from_bytes(data).context(error::MetainfoDeserialize {
      input: source.clone(),
    })?;
    Ok(metainfo)
  }

  pub(crate) fn serialize(&self) -> Result<Vec<u8>, Error> {
    bendy::serde::ser::to_bytes(&self).context(error::MetainfoSerialize)
  }

  #[cfg(test)]
  pub(crate) fn dump(&self, path: impl AsRef<Path>) -> Result<(), Error> {
    let path = path.as_ref();
    let bencode = bendy::serde::ser::to_bytes(&self).context(error::MetainfoSerialize)?;
    fs::write(path, bencode).context(error::Filesystem { path })?;
    Ok(())
  }

  #[cfg(test)]
  pub(crate) fn from_bytes(bytes: &[u8]) -> Metainfo {
    Self::deserialize(&InputTarget::Path("<TEST>".into()), bytes).unwrap()
  }

  #[cfg(test)]
  pub(crate) fn file_paths(&self) -> Vec<String> {
    let files = match &self.info.mode {
      Mode::Single { .. } => panic!(),
      Mode::Multiple { files } => files,
    };

    let paths: Vec<String> = files.iter().map(|f| f.path.to_string()).collect();

    paths
  }

  pub(crate) fn verify(&self, base: &Path, progress_bar: Option<ProgressBar>) -> Result<Status> {
    Verifier::verify(self, base, progress_bar)
  }

  pub(crate) fn content_size(&self) -> Bytes {
    self.info.content_size()
  }

  pub(crate) fn trackers<'a>(&'a self) -> impl Iterator<Item = Result<Url>> + 'a {
    let mut seen = HashSet::new();
    iter::once(&self.announce)
      .flatten()
      .chain(self.announce_list.iter().flatten().flatten())
      .filter_map(move |text| {
        if seen.contains(text) {
          None
        } else {
          seen.insert(text.clone());
          Some(text.parse().context(error::AnnounceUrlParse))
        }
      })
  }

  /// See `Info::infohash_lossy` for details on when this function is lossy.
  pub(crate) fn infohash_lossy(&self) -> Result<Infohash> {
    self.info.infohash_lossy()
  }

  #[cfg(test)]
  pub(crate) fn test_value_single() -> Metainfo {
    Metainfo {
      announce: Some("udp://announce.example:1337".into()),
      announce_list: Some(vec![
        vec![
          "http://a.example:4567".into(),
          "https://b.example:77".into(),
        ],
        vec!["udp://c.example:88".into()],
      ]),
      nodes: Some(vec![
        "node.example:12".parse().unwrap(),
        "1.1.1.1:16".parse().unwrap(),
        "[2001:0db8:85a3::0000:8a2e:0370]:7334".parse().unwrap(),
      ]),
      comment: Some("COMMENT".into()),
      created_by: Some("CREATED BY".into()),
      creation_date: Some(1),
      encoding: Some("UTF-8".into()),
      info: Info {
        private: Some(true),
        piece_length: Bytes(16 * 1024),
        source: Some("SOURCE".into()),
        name: "NAME".into(),
        pieces: PieceList::from_pieces(["fae50", "fae50"]),
        mode: Mode::Single {
          length: Bytes(32 * 1024),
          md5sum: Some(Md5Digest::from_hex("000102030405060708090a0b0c0d0e0f")),
        },
        update_url: Some("https://update.example".parse().unwrap()),
      },
    }
  }

  #[cfg(test)]
  pub(crate) fn test_value_single_infohash() -> &'static str {
    "5d6f53772b4c20536fcce0c4c364d764a6efa39c"
  }

  #[cfg(test)]
  pub(crate) fn test_value_single_torrent_size() -> Bytes {
    Bytes(509)
  }

  #[cfg(test)]
  pub(crate) fn test_value_multiple() -> Metainfo {
    let mut instance = Self::test_value_single();
    instance.info.mode = Mode::Multiple {
      files: vec![FileInfo {
        length: Bytes(32 * 1024),
        path: FilePath::from_components(&["DIR", "FILE"]),
        md5sum: Some(Md5Digest::from_hex("000102030405060708090a0b0c0d0e0f")),
      }],
    };
    instance
  }

  #[cfg(test)]
  pub(crate) fn test_value_single_unset() -> Metainfo {
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
        piece_length: Bytes(1024),
        source: None,
        name: "NAME".into(),
        pieces: PieceList::from_pieces(["fae50"]),
        mode: Mode::Single {
          length: Bytes(5),
          md5sum: None,
        },
        update_url: None,
      },
    }
  }

  #[cfg(test)]
  pub(crate) fn test_value_single_unset_infohash() -> &'static str {
    "a9105b0ff5f7cefeee5599ed7831749be21cc04e"
  }

  #[cfg(test)]
  pub(crate) fn test_value_single_unset_torrent_size() -> Bytes {
    Bytes(85)
  }

  #[cfg(test)]
  pub(crate) fn test_value_multiple_unset() -> Metainfo {
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
        piece_length: Bytes(1024),
        source: None,
        name: "NAME".into(),
        pieces: PieceList::from_pieces(["fae50"]),
        mode: Mode::Multiple {
          files: vec![FileInfo {
            length: Bytes(1024),
            md5sum: None,
            path: FilePath::from_components(&["a", "b"]),
          }],
        },
        update_url: None,
      },
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use pretty_assertions::assert_eq;

  #[test]
  fn round_trip_single() {
    let value = Metainfo::test_value_single();

    let bencode = bendy::serde::ser::to_bytes(&value).unwrap();

    let deserialized = bendy::serde::de::from_bytes(&bencode).unwrap();

    assert_eq!(value, deserialized);
  }

  #[test]
  fn round_trip_multiple() {
    let value = Metainfo::test_value_multiple();

    let bencode = bendy::serde::ser::to_bytes(&value).unwrap();

    let deserialized = bendy::serde::de::from_bytes(&bencode).unwrap();

    assert_eq!(value, deserialized);
  }

  fn representation(value: Metainfo, want: &str) {
    let have = value.serialize().unwrap();

    if have != want.as_bytes() {
      let have = String::from_utf8_lossy(&have);
      assert_eq!(have, want);
      eprintln!("have:");
      eprintln!("{have}");
      eprintln!("want:");
      eprintln!("{want}");
      panic!("Unexpected representation...");
    }
  }

  #[test]
  fn bencode_representation_single_set() {
    let value = Metainfo::test_value_single();

    #[rustfmt::skip]
    let want = concat!(
      "d",
        "8:announce", "27:udp://announce.example:1337",
        "13:announce-list", "l",
          "l",
            "21:http://a.example:4567",
            "20:https://b.example:77",
          "e",
          "l",
            "18:udp://c.example:88", 
          "e",
        "e",
        "7:comment", "7:COMMENT",
        "10:created by", "10:CREATED BY",
        "13:creation date", "i1e",
        "8:encoding", "5:UTF-8",
        "4:info", "d",
          "6:length", "i32768e",
          "6:md5sum", "32:000102030405060708090a0b0c0d0e0f",
          "4:name", "4:NAME",
          "12:piece length", "i16384e",
          "6:pieces", "40:8,OS7d玤{Qk!Mk8,OS7d玤{Qk!Mk",
          "7:private", "i1e",
          "6:source", "6:SOURCE",
          "10:update-url", "23:https://update.example/",
        "e",
        "5:nodes", "l",
          "l", "12:node.example", "i12e", "e",
          "l", "7:1.1.1.1", "i16e", "e",
          "l", "23:2001:db8:85a3::8a2e:370", "i7334e", "e",
        "e",
      "e"
    );

    representation(value, want);
  }

  #[test]
  fn bencode_representation_single_unset() {
    let value = Metainfo::test_value_single_unset();

    #[rustfmt::skip]
    let want = concat!(
      "d",
        "4:info", "d",
          "6:length", "i5e",
          "4:name", "4:NAME",
          "12:piece length", "i1024e",
          "6:pieces", "20:8,OS7d玤{Qk!Mk",
        "e",
      "e"
    );

    representation(value, want);
  }

  #[test]
  fn bencode_representation_multiple_set() {
    let value = Metainfo::test_value_multiple();

    #[rustfmt::skip]
    let want = concat!(
      "d",
        "8:announce", "27:udp://announce.example:1337",
        "13:announce-list", "l",
          "l",
            "21:http://a.example:4567",
            "20:https://b.example:77",
          "e",
          "l",
            "18:udp://c.example:88",
          "e",
        "e",
        "7:comment", "7:COMMENT",
        "10:created by", "10:CREATED BY",
        "13:creation date", "i1e",
        "8:encoding", "5:UTF-8",
        "4:info", "d",
          "5:files", "l",
            "d",
              "6:length", "i32768e",
              "6:md5sum", "32:000102030405060708090a0b0c0d0e0f",
              "4:path", "l", "3:DIR", "4:FILE", "e",
            "e",
          "e",
          "4:name", "4:NAME",
          "12:piece length", "i16384e",
          "6:pieces", "40:8,OS7d玤{Qk!Mk8,OS7d玤{Qk!Mk",
          "7:private", "i1e",
          "6:source", "6:SOURCE",
          "10:update-url", "23:https://update.example/",
        "e",
        "5:nodes", "l",
          "l", "12:node.example", "i12e", "e",
          "l", "7:1.1.1.1", "i16e", "e",
          "l", "23:2001:db8:85a3::8a2e:370", "i7334e", "e",
        "e",
      "e"
    );

    representation(value, want);
  }

  #[test]
  fn bencode_representation_multiple_unset() {
    let value = Metainfo::test_value_multiple_unset();

    #[rustfmt::skip]
    let want = concat!(
      "d",
        "4:info", "d",
          "5:files", "l",
            "d",
              "6:length", "i1024e",
              "4:path", "l", "1:a", "1:b", "e",
            "e",
          "e",
          "4:name", "4:NAME",
          "12:piece length", "i1024e",
          "6:pieces", "20:8,OS7d玤{Qk!Mk",
        "e",
      "e"
    );

    representation(value, want);
  }

  #[test]
  fn trackers() {
    fn assert_trackers_eq(metainfo: &Metainfo, want: &[&str]) {
      let want = want
        .iter()
        .copied()
        .map(Url::parse)
        .collect::<Result<Vec<Url>, url::ParseError>>()
        .unwrap();
      let have = metainfo.trackers().collect::<Result<Vec<Url>>>().unwrap();
      assert_eq!(have, want);
    }

    let mut metainfo = Metainfo::test_value_single();

    assert_trackers_eq(
      &metainfo,
      &[
        "udp://announce.example:1337",
        "http://a.example:4567",
        "https://b.example:77",
        "udp://c.example:88",
      ],
    );

    metainfo.announce_list = Some(vec![
      vec![
        "udp://announce.example:1337".into(),
        "https://b.example:77".into(),
      ],
      vec!["udp://c.example:88".into()],
    ]);

    assert_trackers_eq(
      &metainfo,
      &[
        "udp://announce.example:1337",
        "https://b.example:77",
        "udp://c.example:88",
      ],
    );
  }
}
