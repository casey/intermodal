use crate::common::*;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub(crate) struct FileInfo {
  pub(crate) length: u64,
  pub(crate) path: FilePath,
  #[serde(skip_serializing_if = "Option::is_none", default, with = "inner")]
  pub(crate) md5sum: Option<String>,
}
