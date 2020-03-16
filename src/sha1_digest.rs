use crate::common::*;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub(crate) struct Sha1Digest {
  bytes: [u8; Self::LENGTH],
}

impl Sha1Digest {
  pub(crate) const LENGTH: usize = 20;

  pub(crate) fn from_bytes(bytes: [u8; Self::LENGTH]) -> Self {
    Sha1Digest { bytes }
  }

  pub(crate) fn bytes(self) -> [u8; Self::LENGTH] {
    self.bytes
  }

  #[cfg(test)]
  pub(crate) fn from_data(data: impl AsRef<[u8]>) -> Self {
    Sha1::from(data).digest().into()
  }
}

impl From<sha1::Digest> for Sha1Digest {
  fn from(digest: sha1::Digest) -> Self {
    Self {
      bytes: digest.bytes(),
    }
  }
}

impl Display for Sha1Digest {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    for byte in &self.bytes {
      write!(f, "{:x}", byte)?;
    }

    Ok(())
  }
}
