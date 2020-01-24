use crate::common::*;

#[derive(Deserialize, Serialize)]
pub struct Metainfo {
  pub announce: String,
  #[serde(rename = "announce list")]
  pub announce_list: Option<Vec<Vec<String>>>,
  pub comment: Option<String>,
  #[serde(rename = "created by")]
  pub created_by: Option<String>,
  #[serde(rename = "creation date")]
  pub creation_date: Option<u64>,
  pub encoding: String,
  pub info: Info,
}
