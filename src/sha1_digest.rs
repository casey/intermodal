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
}

impl From<sha1::Digest> for Sha1Digest {
  fn from(digest: sha1::Digest) -> Self {
    Self {
      bytes: digest.bytes(),
    }
  }
}
