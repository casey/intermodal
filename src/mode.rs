use crate::common::*;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum Mode {
  Single { length: u64, md5sum: Option<String> },
  Multiple { files: Vec<FileInfo> },
}
