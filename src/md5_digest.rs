use crate::common::*;

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Copy, Clone)]
#[serde(transparent)]
pub(crate) struct Md5Digest {
  #[serde(with = "SerHex::<serde_hex::Strict>")]
  bytes: [u8; 16],
}

impl Md5Digest {
  #[cfg(test)]
  pub(crate) fn from_hex(hex: &str) -> Self {
    assert_eq!(hex.len(), 32);

    let mut bytes: [u8; 16] = [0; 16];

    for (n, byte) in bytes.iter_mut().enumerate() {
      let i = n * 2;
      *byte = u8::from_str_radix(&hex[i..i + 2], 16).unwrap();
    }

    Self { bytes }
  }

  #[cfg(test)]
  pub(crate) fn from_data(data: impl AsRef<[u8]>) -> Self {
    md5::compute(data).into()
  }
}

impl From<md5::Digest> for Md5Digest {
  fn from(digest: md5::Digest) -> Self {
    Self { bytes: digest.0 }
  }
}

impl Display for Md5Digest {
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
  fn ser() {
    let digest = Md5Digest {
      bytes: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    };

    let bytes = bendy::serde::ser::to_bytes(&digest).unwrap();

    assert_eq!(
      str::from_utf8(&bytes).unwrap(),
      "32:000102030405060708090a0b0c0d0e0f"
    );

    let string_bytes = bendy::serde::ser::to_bytes(&"000102030405060708090a0b0c0d0e0f").unwrap();

    assert_eq!(bytes, string_bytes);
  }

  #[test]
  fn display() {
    let digest = Md5Digest {
      bytes: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    };

    assert_eq!(digest.to_string(), "000102030405060708090a0b0c0d0e0f");
  }
}
