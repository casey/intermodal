use crate::common::*;

#[derive(Deserialize, Serialize)]
pub(crate) struct Info {
  pub(crate) private: Option<u8>,
  #[serde(rename = "piece length")]
  pub(crate) piece_length: u32,
  pub(crate) name: String,
  pub(crate) source: Option<String>,
  #[serde(with = "serde_bytes")]
  pub(crate) pieces: Vec<u8>,
  #[serde(flatten)]
  pub(crate) mode: Mode,
}
