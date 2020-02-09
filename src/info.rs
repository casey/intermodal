use crate::common::*;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub(crate) struct Info {
  #[serde(skip_serializing_if = "Option::is_none", default, with = "inner")]
  pub(crate) private: Option<u8>,
  #[serde(rename = "piece length")]
  pub(crate) piece_length: u32,
  pub(crate) name: String,
  #[serde(skip_serializing_if = "Option::is_none", default, with = "inner")]
  pub(crate) source: Option<String>,
  #[serde(with = "serde_bytes")]
  pub(crate) pieces: Vec<u8>,
  #[serde(flatten)]
  pub(crate) mode: Mode,
}
