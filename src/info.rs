use crate::common::*;

#[derive(Deserialize, Serialize)]
pub struct Info {
  pub private: u8,
  #[serde(rename = "piece length")]
  pub piece_length: u32,
  pub name: String,
  #[serde(with = "serde_bytes")]
  pub pieces: Vec<u8>,
  #[serde(flatten)]
  pub mode: Mode,
}
