use crate::common::*;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub(crate) struct FileInfo {
  pub(crate) length: u64,
  pub(crate) md5sum: Option<String>,
  pub(crate) path: FilePath,
}
