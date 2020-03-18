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
  pub(crate) fn load(path: impl AsRef<Path>) -> Result<Metainfo, Error> {
    let path = path.as_ref();
    let bytes = fs::read(path).context(error::Filesystem { path })?;
    Self::deserialize(path, &bytes)
  }

  pub(crate) fn deserialize(path: impl AsRef<Path>, bytes: &[u8]) -> Result<Metainfo, Error> {
    let path = path.as_ref();
    let metainfo =
      bendy::serde::de::from_bytes(&bytes).context(error::MetainfoDeserialize { path })?;
    Ok(metainfo)
  }

  pub(crate) fn serialize(&self) -> Result<Vec<u8>, Error> {
    bendy::serde::ser::to_bytes(&self).context(error::MetainfoSerialize)
  }

  #[cfg(test)]
  pub(crate) fn dump(&self, path: impl AsRef<Path>) -> Result<(), Error> {
    let path = path.as_ref();
    let bencode = bendy::serde::ser::to_bytes(&self).context(error::MetainfoSerialize)?;
    fs::write(path, &bencode).context(error::Filesystem { path })?;
    Ok(())
  }

  #[cfg(test)]
  pub(crate) fn from_bytes(bytes: &[u8]) -> Metainfo {
    Self::deserialize("<TEST>", bytes).unwrap()
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
      .filter(move |text| {
        if seen.contains(text) {
          false
        } else {
          seen.insert(text.clone());
          true
        }
      })
      .map(|text| text.parse().context(error::AnnounceUrlParse))
  }

  pub(crate) fn infohash(&self) -> Result<Infohash> {
    self.info.infohash()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn round_trip_single() {
    let value = Metainfo {
      announce: Some("announce".into()),
      announce_list: Some(vec![vec!["announce".into(), "b".into()], vec!["c".into()]]),
      comment: Some("comment".into()),
      created_by: Some("created by".into()),
      creation_date: Some(1),
      encoding: Some("UTF-8".into()),
      nodes: Some(vec!["x:12".parse().unwrap(), "1.1.1.1:16".parse().unwrap()]),
      info: Info {
        private: Some(true),
        piece_length: Bytes(16 * 1024),
        source: Some("source".into()),
        name: "foo".into(),
        pieces: PieceList::from_pieces(&["abc"]),
        mode: Mode::Single {
          length: Bytes(20),
          md5sum: None,
        },
      },
    };

    let bencode = bendy::serde::ser::to_bytes(&value).unwrap();

    let deserialized = bendy::serde::de::from_bytes(&bencode).unwrap();

    assert_eq!(value, deserialized);
  }

  #[test]
  fn round_trip_multiple() {
    let value = Metainfo {
      announce: Some("announce".into()),
      announce_list: Some(vec![vec!["announce".into(), "b".into()], vec!["c".into()]]),
      nodes: Some(vec!["x:12".parse().unwrap(), "1.1.1.1:16".parse().unwrap()]),
      comment: Some("comment".into()),
      created_by: Some("created by".into()),
      creation_date: Some(1),
      encoding: Some("UTF-8".into()),
      info: Info {
        private: Some(true),
        piece_length: Bytes(16 * 1024),
        source: Some("source".into()),
        name: "foo".into(),
        pieces: PieceList::from_pieces(&["abc"]),
        mode: Mode::Multiple {
          files: vec![FileInfo {
            length: Bytes(10),
            path: FilePath::from_components(&["foo", "bar"]),
            md5sum: Some(Md5Digest::from_hex("000102030405060708090a0b0c0d0e0f")),
          }],
        },
      },
    };

    let bencode = bendy::serde::ser::to_bytes(&value).unwrap();

    let deserialized = bendy::serde::de::from_bytes(&bencode).unwrap();

    assert_eq!(value, deserialized);
  }

  fn representation(value: Metainfo, want: &str) {
    let have = value.serialize().unwrap();

    if have != want.as_bytes() {
      eprintln!("have:");
      eprintln!("{}", String::from_utf8_lossy(&have));
      eprintln!("want:");
      eprintln!("{}", want);
      panic!("Unexpected representation...");
    }
  }

  #[test]
  fn bencode_representation_single_some() {
    let value = Metainfo {
      announce: Some("ANNOUNCE".into()),
      announce_list: Some(vec![vec!["A".into(), "B".into()], vec!["C".into()]]),
      nodes: Some(vec![
        "domain:1".parse().unwrap(),
        "1.1.1.1:16".parse().unwrap(),
        "[1234:5678:9abc:def0:1234:5678:9abc:def0]:65000"
          .parse()
          .unwrap(),
      ]),
      comment: Some("COMMENT".into()),
      created_by: Some("CREATED BY".into()),
      creation_date: Some(0),
      encoding: Some("UTF-8".into()),
      info: Info {
        private: Some(true),
        piece_length: Bytes(1024),
        source: Some("SOURCE".into()),
        name: "NAME".into(),
        pieces: PieceList::from_pieces(&["fae50"]),
        mode: Mode::Single {
          length: Bytes(5),
          md5sum: Some(Md5Digest::from_hex("000102030405060708090a0b0c0d0e0f")),
        },
      },
    };

    #[rustfmt::skip]
    let want = concat!(
      "d",
        "8:announce", "8:ANNOUNCE",
        "13:announce-list", "l", 
          "l", "1:A", "1:B", "e",
          "l", "1:C", "e",
        "e",
        "7:comment", "7:COMMENT",
        "10:created by", "10:CREATED BY",
        "13:creation date", "i0e",
        "8:encoding", "5:UTF-8",
        "4:info", "d",
          "6:length", "i5e",
          "6:md5sum", "32:000102030405060708090a0b0c0d0e0f",
          "4:name", "4:NAME",
          "12:piece length", "i1024e",
          "6:pieces", "20:8,OS7d玤{Qk!Mk",
          "7:private", "i1e",
          "6:source", "6:SOURCE",
        "e",
        "5:nodes", "l",
          "l", "6:domain", "i1e", "e",
          "l", "7:1.1.1.1", "i16e", "e",
          "l", "39:1234:5678:9abc:def0:1234:5678:9abc:def0", "i65000e", "e",
        "e",
      "e"
    );

    representation(value, want);
  }

  #[test]
  fn bencode_representation_single_none() {
    let value = Metainfo {
      announce: Some("ANNOUNCE".into()),
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
        pieces: PieceList::from_pieces(&["fae50"]),
        mode: Mode::Single {
          length: Bytes(5),
          md5sum: None,
        },
      },
    };

    #[rustfmt::skip]
    let want = concat!(
      "d",
        "8:announce", "8:ANNOUNCE",
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
  fn bencode_representation_multiple_some() {
    let value = Metainfo {
      announce: Some("ANNOUNCE".into()),
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
        pieces: PieceList::from_pieces(&["fae50"]),
        mode: Mode::Multiple {
          files: vec![FileInfo {
            length: Bytes(1024),
            md5sum: Some(Md5Digest::from_hex("000102030405060708090a0b0c0d0e0f")),
            path: FilePath::from_components(&["a", "b"]),
          }],
        },
      },
    };

    #[rustfmt::skip]
    let want = concat!(
      "d",
        "8:announce", "8:ANNOUNCE",
        "4:info", "d",
          "5:files", "l",
            "d",
              "6:length", "i1024e",
              "6:md5sum", "32:000102030405060708090a0b0c0d0e0f",
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
  fn bencode_representation_multiple_none() {
    let value = Metainfo {
      announce: Some("ANNOUNCE".into()),
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
        pieces: PieceList::from_pieces(&["fae50"]),
        mode: Mode::Multiple {
          files: vec![FileInfo {
            length: Bytes(1024),
            md5sum: None,
            path: FilePath::from_components(&["a", "b"]),
          }],
        },
      },
    };

    #[rustfmt::skip]
    let want = concat!(
      "d",
        "8:announce", "8:ANNOUNCE",
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
  fn private_false() {
    let value = Metainfo {
      announce: Some("ANNOUNCE".into()),
      announce_list: None,
      nodes: None,
      comment: None,
      created_by: None,
      creation_date: None,
      encoding: None,
      info: Info {
        private: Some(false),
        piece_length: Bytes(1024),
        source: None,
        name: "NAME".into(),
        pieces: PieceList::from_pieces(&["fae50"]),
        mode: Mode::Single {
          length: Bytes(5),
          md5sum: None,
        },
      },
    };

    #[rustfmt::skip]
    let want = concat!(
      "d",
        "8:announce", "8:ANNOUNCE",
        "4:info", "d",
          "6:length", "i5e",
          "4:name", "4:NAME",
          "12:piece length", "i1024e",
          "6:pieces", "20:8,OS7d玤{Qk!Mk",
          "7:private", "i0e",
        "e",
      "e"
    );

    representation(value, want);
  }

  #[test]
  fn trackers() {
    let mut metainfo = Metainfo {
      announce: Some("http://foo".into()),
      announce_list: None,
      nodes: None,
      comment: None,
      created_by: None,
      creation_date: None,
      encoding: None,
      info: Info {
        private: Some(false),
        piece_length: Bytes(1024),
        source: None,
        name: "NAME".into(),
        pieces: PieceList::from_pieces(&["fae50"]),
        mode: Mode::Single {
          length: Bytes(5),
          md5sum: None,
        },
      },
    };

    let trackers = metainfo.trackers().collect::<Result<Vec<Url>>>().unwrap();
    assert_eq!(trackers, &["http://foo".parse().unwrap()]);

    metainfo.announce_list = Some(vec![
      vec!["http://bar".into(), "http://baz".into()],
      vec!["http://foo".into()],
    ]);

    let trackers = metainfo.trackers().collect::<Result<Vec<Url>>>().unwrap();
    assert_eq!(
      trackers,
      &[
        "http://foo".parse().unwrap(),
        "http://bar".parse().unwrap(),
        "http://baz".parse().unwrap(),
      ],
    );
  }
}
