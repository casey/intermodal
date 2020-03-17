use crate::*;

pub(crate) struct Infohash {
  inner: Sha1Digest,
}

impl Infohash {
  pub(crate) fn load(path: &Path) -> Result<Infohash, Error> {
    use bendy::decoding::{Decoder, Object};

    let bytes = fs::read(path).context(error::Filesystem { path })?;

    let mut decoder = Decoder::new(&bytes);

    let object = decoder.next_object().unwrap().unwrap();

    if let Object::Dict(mut decoder) = object {
      loop {
        let (key, val) = decoder.next_pair().unwrap().unwrap();

        if key == b"info" {
          if let Object::Dict(infodict) = val {
            let raw = infodict.into_raw().unwrap();
            trace!(
              "bencoded info dictionary: `{}`",
              String::from_utf8_lossy(raw)
            );
            return Ok(Infohash {
              inner: Sha1Digest::from_data(raw),
            });
          }
        }
      }
    }

    panic!()
  }

  #[cfg(test)]
  pub(crate) fn from_data(bytes: impl AsRef<[u8]>) -> Infohash {
    Infohash {
      inner: Sha1Digest::from_data(bytes),
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
