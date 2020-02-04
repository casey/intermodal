use crate::common::*;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum Mode {
  Single { length: u64, md5sum: Option<String> },
  Multiple { files: Vec<FileInfo> },
}

impl Mode {
  pub(crate) fn total_size(&self) -> Bytes {
    match self {
      Self::Single { length, .. } => Bytes::from(*length),
      Self::Multiple { files } => Bytes::from(files.iter().map(|file| file.length).sum::<u64>()),
    }
  }
}
