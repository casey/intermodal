use crate::common::*;

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub(crate) struct Info {
  #[serde(
    skip_serializing_if = "Option::is_none",
    default,
    with = "unwrap_or_skip"
  )]
  pub(crate) private: Option<bool>,
  #[serde(rename = "piece length")]
  pub(crate) piece_length: Bytes,
  pub(crate) name: String,
  #[serde(
    skip_serializing_if = "Option::is_none",
    default,
    with = "unwrap_or_skip"
  )]
  pub(crate) source: Option<String>,
  #[serde(with = "serde_bytes")]
  pub(crate) pieces: Vec<u8>,
  #[serde(flatten)]
  pub(crate) mode: Mode,
}
