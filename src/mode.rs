use crate::common::*;

#[skip_serializing_none]
#[serde(untagged)]
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub(crate) enum Mode {
  Single {
    length: u64,
    #[serde(skip_serializing_if = "Option::is_none", default, with = "inner")]
    md5sum: Option<String>,
  },
  Multiple {
    files: Vec<FileInfo>,
  },
}

impl Mode {
  pub(crate) fn total_size(&self) -> Bytes {
    match self {
      Self::Single { length, .. } => Bytes::from(*length),
      Self::Multiple { files } => Bytes::from(files.iter().map(|file| file.length).sum::<u64>()),
    }
  }
}
