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
  pub(crate) pieces: PieceList,
  #[serde(flatten)]
  pub(crate) mode: Mode,
  #[serde(
    skip_serializing_if = "Option::is_none",
    default,
    with = "unwrap_or_skip",
    rename = "update-url"
  )]
  pub(crate) update_url: Option<Url>,
}

impl Info {
  pub(crate) fn content_size(&self) -> Bytes {
    self.mode.content_size()
  }

  pub(crate) fn infohash(&self) -> Result<Infohash> {
    let encoded = bendy::serde::ser::to_bytes(self).context(error::InfoSerialize)?;
    Ok(Infohash::from_bencoded_info_dict(&encoded))
  }
}
