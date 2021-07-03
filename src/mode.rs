use crate::common::*;

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub(crate) enum Mode {
  Single {
    length: Bytes,
    #[serde(
      skip_serializing_if = "Option::is_none",
      default,
      with = "unwrap_or_skip"
    )]
    md5sum: Option<Md5Digest>,
  },
  Multiple {
    files: Vec<FileInfo>,
  },
}

impl Mode {
  pub(crate) fn content_size(&self) -> Bytes {
    match self {
      Self::Single { length, .. } => *length,
      Self::Multiple { files } => files.iter().map(|file| file.length).sum(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn single_no_md5sum() {
    let input = Mode::Single {
      length: Bytes(10),
      md5sum: None,
    };

    let have = bendy::serde::ser::to_bytes(&input).unwrap();

    assert_eq!(str::from_utf8(&have).unwrap(), "d6:lengthi10ee");

    let output: Mode = bendy::serde::de::from_bytes(&have).unwrap();

    assert_eq!(output, input);
  }

  #[test]
  fn single_with_md5sum() {
    let input = Mode::Single {
      length: Bytes(10),
      md5sum: Some(Md5Digest::from_hex("000102030405060708090a0b0c0d0e0f")),
    };

    let have = bendy::serde::ser::to_bytes(&input).unwrap();

    assert_eq!(
      str::from_utf8(&have).unwrap(),
      "d6:lengthi10e6:md5sum32:000102030405060708090a0b0c0d0e0fe"
    );

    let output: Mode = bendy::serde::de::from_bytes(&have).unwrap();

    assert_eq!(output, input);
  }

  #[test]
  fn round_trip_single() {
    let value = Mode::Single {
      length: Bytes(10),
      md5sum: Some(Md5Digest::from_hex("000102030405060708090a0b0c0d0e0f")),
    };

    let bencode = bendy::serde::ser::to_bytes(&value).unwrap();

    let deserialized = bendy::serde::de::from_bytes(&bencode).unwrap();

    assert_eq!(value, deserialized);
  }

  #[test]
  fn round_trip_multiple() {
    let value = Mode::Multiple {
      files: vec![FileInfo {
        length: Bytes(10),
        path: FilePath::from_components(&["foo", "bar"]),
        md5sum: Some(Md5Digest::from_hex("000102030405060708090a0b0c0d0e0f")),
      }],
    };

    let bencode = bendy::serde::ser::to_bytes(&value).unwrap();

    let deserialized = bendy::serde::de::from_bytes(&bencode).unwrap();

    assert_eq!(value, deserialized);
  }
}
