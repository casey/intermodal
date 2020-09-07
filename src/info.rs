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

  /// This function is potentially lossy. If an arbitrary torrent info
  /// dictionary is deserialized into an `Info` struct, extra fields not present
  /// on the struct will be discarded. If `infohash_lossy` is then called on
  /// the resultant struct, those fields will not contribute to the infohash,
  /// which will thus be different from that of the original torrent.
  ///
  /// It will not be lossy if no extra fields are present in the original
  /// torrent. So, it is safe to call on torrents that have just been created
  /// and are still in memory, and thus are known to have no extra fields.
  pub(crate) fn infohash_lossy(&self) -> Result<Infohash> {
    let encoded = bendy::serde::ser::to_bytes(self).context(error::InfoSerialize)?;
    Ok(Infohash::from_bencoded_info_dict(&encoded))
  }
}
