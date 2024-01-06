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
      write!(f, "{byte:02x}")?;
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn display() {
    let digest = Sha1Digest {
      bytes: [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
      ],
    };

    assert_eq!(
      digest.to_string(),
      "000102030405060708090a0b0c0d0e0f10111213"
    );
  }
}
