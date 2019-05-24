use crate::common::*;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct FileInfo {
  pub length: u64,
  pub md5sum: Option<String>,
  pub path: Vec<String>,
}
