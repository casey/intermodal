use crate::common::*;

#[derive(Deserialize, Serialize)]
pub(crate) struct Metainfo {
  pub(crate) announce: String,
  #[serde(rename = "announce-list")]
  pub(crate) announce_list: Option<Vec<Vec<String>>>,
  pub(crate) comment: Option<String>,
  #[serde(rename = "created by")]
  pub(crate) created_by: Option<String>,
  #[serde(rename = "creation date")]
  pub(crate) creation_date: Option<u64>,
  pub(crate) encoding: Option<String>,
  pub(crate) info: Info,
}

impl Metainfo {
  pub(crate) fn _load(path: impl AsRef<Path>) -> Result<Metainfo, Error> {
    let path = path.as_ref();
    let bytes = fs::read(path).context(error::Filesystem { path })?;
    Self::deserialize(path, &bytes)
  }

  #[cfg(test)]
  pub(crate) fn dump(&self, path: impl AsRef<Path>) -> Result<(), Error> {
    let path = path.as_ref();
    let bytes = serde_bencode::ser::to_bytes(&self).context(error::MetainfoSerialize)?;
    fs::write(path, &bytes).context(error::Filesystem { path })?;
    Ok(())
  }

  pub(crate) fn deserialize(path: impl AsRef<Path>, bytes: &[u8]) -> Result<Metainfo, Error> {
    let path = path.as_ref();
    serde_bencode::de::from_bytes(&bytes).context(error::MetainfoLoad { path })
  }

  pub(crate) fn serialize(&self) -> Result<Vec<u8>, Error> {
    serde_bencode::ser::to_bytes(&self).context(error::MetainfoSerialize)
  }
}
