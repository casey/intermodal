use crate::common::*;

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub(crate) struct Metainfo {
  pub(crate) announce: String,
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

  #[cfg(test)]
  pub(crate) fn dump(&self, path: impl AsRef<Path>) -> Result<(), Error> {
    let path = path.as_ref();
    let bencode = bendy::serde::ser::to_bytes(&self).context(error::MetainfoSerialize)?;
    fs::write(path, &bencode).context(error::Filesystem { path })?;
    Ok(())
  }

  pub(crate) fn deserialize(path: impl AsRef<Path>, bytes: &[u8]) -> Result<Metainfo, Error> {
    let path = path.as_ref();
    bendy::serde::de::from_bytes(&bytes).context(error::MetainfoLoad { path })
  }

  pub(crate) fn serialize(&self) -> Result<Vec<u8>, Error> {
    bendy::serde::ser::to_bytes(&self).context(error::MetainfoSerialize)
  }

  #[cfg(test)]
  pub(crate) fn from_bytes(bytes: &[u8]) -> Metainfo {
    bendy::serde::de::from_bytes(bytes).unwrap()
  }

  pub(crate) fn files<'a>(
    &'a self,
    base: &'a Path,
  ) -> Box<dyn Iterator<Item = (PathBuf, Bytes, Option<Md5Digest>)> + 'a> {
    match &self.info.mode {
      Mode::Single { length, md5sum } => Box::new(iter::once((base.to_owned(), *length, *md5sum))),
      Mode::Multiple { files } => {
        let base = base.to_owned();
        Box::new(
          files
            .iter()
            .map(move |file| (file.path.absolute(&base), file.length, file.md5sum)),
        )
      }
    }
  }

  pub(crate) fn verify(&self, base: &Path) -> Result<Status> {
    Verifier::verify(self, base)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn round_trip_single() {
    let value = Metainfo {
      announce: "announce".into(),
      announce_list: Some(vec![vec!["announce".into(), "b".into()], vec!["c".into()]]),
      comment: Some("comment".into()),
      created_by: Some("created by".into()),
      creation_date: Some(1),
      encoding: Some("UTF-8".into()),
      info: Info {
        private: Some(true),
        piece_length: Bytes(16 * 1024),
        source: Some("source".into()),
        name: "foo".into(),
        pieces: vec![
          0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
        ],
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
      announce: "announce".into(),
      announce_list: Some(vec![vec!["announce".into(), "b".into()], vec!["c".into()]]),
      comment: Some("comment".into()),
      created_by: Some("created by".into()),
      creation_date: Some(1),
      encoding: Some("UTF-8".into()),
      info: Info {
        private: Some(true),
        piece_length: Bytes(16 * 1024),
        source: Some("source".into()),
        name: "foo".into(),
        pieces: vec![
          0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
        ],
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
}
