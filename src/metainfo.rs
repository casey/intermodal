use crate::common::*;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub(crate) struct Metainfo {
  pub(crate) announce: String,
  #[serde(rename = "announce-list")]
  #[serde(skip_serializing_if = "Option::is_none", default, with = "inner")]
  pub(crate) announce_list: Option<Vec<Vec<String>>>,
  #[serde(skip_serializing_if = "Option::is_none", default, with = "inner")]
  pub(crate) comment: Option<String>,
  #[serde(rename = "created by")]
  #[serde(skip_serializing_if = "Option::is_none", default, with = "inner")]
  pub(crate) created_by: Option<String>,
  #[serde(rename = "creation date")]
  #[serde(skip_serializing_if = "Option::is_none", default, with = "inner")]
  pub(crate) creation_date: Option<u64>,
  #[serde(skip_serializing_if = "Option::is_none", default, with = "inner")]
  pub(crate) encoding: Option<String>,
  pub(crate) info: Info,
}

impl Metainfo {
  #[cfg(test)]
  pub(crate) fn load(path: impl AsRef<Path>) -> Result<Metainfo, Error> {
    let path = path.as_ref();
    let bytes = fs::read(path).context(error::Filesystem { path })?;
    Self::deserialize(path, &bytes)
  }

  #[cfg(test)]
  pub(crate) fn dump(&self, path: impl AsRef<Path>) -> Result<(), Error> {
    let path = path.as_ref();
    let bendy = bendy::serde::ser::to_bytes(&self).context(error::MetainfoSerialize)?;
    let serde_bencode = serde_bencode::ser::to_bytes(&self).unwrap();
    if bendy != serde_bencode {
      panic!(
        "Serialize bendy != serde_bencode:\n{}\n{}",
        String::from_utf8_lossy(&bendy),
        String::from_utf8_lossy(&serde_bencode)
      );
    }
    fs::write(path, &bendy).context(error::Filesystem { path })?;
    Ok(())
  }

  pub(crate) fn deserialize(path: impl AsRef<Path>, bytes: &[u8]) -> Result<Metainfo, Error> {
    let path = path.as_ref();
    let bendy = bendy::serde::de::from_bytes(&bytes).context(error::MetainfoLoad { path })?;
    let serde_bencode = serde_bencode::de::from_bytes(&bytes).unwrap();
    assert_eq!(bendy, serde_bencode);
    Ok(bendy)
  }

  pub(crate) fn serialize(&self) -> Result<Vec<u8>, Error> {
    let bendy = bendy::serde::ser::to_bytes(&self).context(error::MetainfoSerialize)?;
    let serde_bencode = serde_bencode::ser::to_bytes(&self).unwrap();
    if bendy != serde_bencode {
      panic!(
        "Serialize bendy != serde_bencode:\n{}\n{}",
        String::from_utf8_lossy(&bendy),
        String::from_utf8_lossy(&serde_bencode)
      );
    }
    Ok(bendy)
  }

  #[cfg(test)]
  pub(crate) fn from_bytes(bytes: &[u8]) -> Metainfo {
    let bendy = bendy::serde::de::from_bytes(bytes).unwrap();
    let serde_bencode = serde_bencode::de::from_bytes(bytes).unwrap();
    assert_eq!(bendy, serde_bencode);
    bendy
  }
}
