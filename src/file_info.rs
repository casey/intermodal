use crate::common::*;

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub(crate) struct FileInfo {
  pub(crate) length: Bytes,
  pub(crate) path: FilePath,
  #[serde(
    skip_serializing_if = "Option::is_none",
    default,
    with = "unwrap_or_skip"
  )]
  pub(crate) md5sum: Option<Md5Digest>,
}
