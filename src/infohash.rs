use crate::*;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub(crate) struct Infohash {
  inner: Sha1Digest,
}

impl Infohash {
  pub(crate) fn load(path: &Path) -> Result<Infohash, Error> {
    let bytes = fs::read(path).context(error::Filesystem { path })?;

    let value = Value::from_bencode(&bytes).map_err(|error| Error::MetainfoDecode {
      path: path.to_owned(),
      error,
    })?;

    match value {
      Value::Dict(metainfo) => {
        let info = metainfo
          .iter()
          .find(|pair: &(&Cow<[u8]>, &Value)| pair.0.as_ref() == b"info")
          .ok_or_else(|| Error::MetainfoValidate {
            path: path.to_owned(),
            source: MetainfoError::InfoMissing,
          })?
          .1;

        if let Value::Dict(_) = info {
          let encoded = info.to_bencode().map_err(|error| {
            Error::internal(format!("Failed to re-encode info dictionary: {}", error))
          })?;

          Ok(Self::from_bencoded_info_dict(&encoded))
        } else {
          Err(Error::MetainfoValidate {
            path: path.to_owned(),
            source: MetainfoError::InfoType,
          })
        }
      }
      _ => Err(Error::MetainfoValidate {
        path: path.to_owned(),
        source: MetainfoError::Type,
      }),
    }
  }

  pub(crate) fn from_bencoded_info_dict(info: &[u8]) -> Infohash {
    Infohash {
      inner: Sha1Digest::from_data(info),
    }
  }
}

impl Into<Sha1Digest> for Infohash {
  fn into(self) -> Sha1Digest {
    self.inner
  }
}

impl Display for Infohash {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{}", self.inner)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn load_invalid() {
    let tempdir = temptree! {
      foo: "x",
    };

    let input = tempdir.path().join("foo");

    assert_matches!(
      Infohash::load(&input),
      Err(Error::MetainfoDecode{path, .. })
      if path == input
    );
  }

  #[test]
  fn load_wrong_type() {
    let tempdir = temptree! {
      foo: "i0e",
    };

    let input = tempdir.path().join("foo");

    assert_matches!(
      Infohash::load(&input),
      Err(Error::MetainfoValidate{path, source: MetainfoError::Type})
      if path == input
    );
  }

  #[test]
  fn load_no_info() {
    let tempdir = temptree! {
      foo: "de",
    };

    let input = tempdir.path().join("foo");

    assert_matches!(
      Infohash::load(&input),
      Err(Error::MetainfoValidate{path, source: MetainfoError::InfoMissing})
      if path == input
    );
  }

  #[test]
  fn load_info_type() {
    let tempdir = temptree! {
      foo: "d4:infoi0ee",
    };

    let input = tempdir.path().join("foo");

    assert_matches!(
      Infohash::load(&input),
      Err(Error::MetainfoValidate{path, source: MetainfoError::InfoType})
      if path == input
    );
  }
}
