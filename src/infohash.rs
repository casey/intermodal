use crate::common::*;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub(crate) struct Infohash {
  inner: Sha1Digest,
}

impl Infohash {
  pub(crate) fn from_input(input: &Input) -> Result<Infohash, Error> {
    let value = Value::from_bencode(&input.data).map_err(|error| Error::MetainfoDecode {
      input: input.source.clone(),
      error,
    })?;

    match value {
      Value::Dict(metainfo) => {
        let info = metainfo
          .iter()
          .find(|pair: &(&Cow<[u8]>, &Value)| pair.0.as_ref() == b"info")
          .ok_or_else(|| Error::MetainfoValidate {
            input: input.source.clone(),
            source: MetainfoError::InfoMissing,
          })?
          .1;

        if let Value::Dict(_) = info {
          let encoded = info.to_bencode().map_err(|error| {
            Error::internal(format!("Failed to re-encode info dictionary: {error}"))
          })?;

          Ok(Self::from_bencoded_info_dict(&encoded))
        } else {
          Err(Error::MetainfoValidate {
            input: input.source.clone(),
            source: MetainfoError::InfoType,
          })
        }
      }
      _ => Err(Error::MetainfoValidate {
        input: input.source.clone(),
        source: MetainfoError::Type,
      }),
    }
  }

  pub(crate) fn from_bencoded_info_dict(info: &[u8]) -> Infohash {
    Infohash {
      inner: Sha1Digest::from_data(info),
    }
  }

  #[cfg(test)]
  pub(crate) fn load(path: &Path) -> Result<Infohash, Error> {
    let input = Input::from_path(path)?;
    Self::from_input(&input)
  }
}

impl From<Sha1Digest> for Infohash {
  fn from(inner: Sha1Digest) -> Self {
    Self { inner }
  }
}

impl From<[u8; 20]> for Infohash {
  fn from(bytes: [u8; 20]) -> Self {
    Infohash {
      inner: Sha1Digest::from_bytes(bytes),
    }
  }
}

impl From<Infohash> for [u8; 20] {
  fn from(infohash: Infohash) -> Self {
    infohash.inner.bytes()
  }
}

impl From<Infohash> for Sha1Digest {
  fn from(infohash: Infohash) -> Sha1Digest {
    infohash.inner
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

    let path = tempdir.path().join("foo");

    assert_matches!(
      Infohash::load(&path),
      Err(Error::MetainfoDecode{input, .. })
      if input == path
    );
  }

  #[test]
  fn load_wrong_type() {
    let tempdir = temptree! {
      foo: "i0e",
    };

    let path = tempdir.path().join("foo");

    assert_matches!(
      Infohash::load(&path),
      Err(Error::MetainfoValidate{input, source: MetainfoError::Type})
      if input == path
    );
  }

  #[test]
  fn load_no_info() {
    let tempdir = temptree! {
      foo: "de",
    };

    let path = tempdir.path().join("foo");

    assert_matches!(
      Infohash::load(&path),
      Err(Error::MetainfoValidate{input, source: MetainfoError::InfoMissing})
      if input == path
    );
  }

  #[test]
  fn load_info_type() {
    let tempdir = temptree! {
      foo: "d4:infoi0ee",
    };

    let path = tempdir.path().join("foo");

    assert_matches!(
      Infohash::load(&path),
      Err(Error::MetainfoValidate{input, source: MetainfoError::InfoType})
      if input == path
    );
  }
}
