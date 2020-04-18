use crate::common::*;

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct PieceList {
  piece_hashes: Vec<Sha1Digest>,
}

impl PieceList {
  pub(crate) fn new() -> Self {
    Self {
      piece_hashes: Vec::new(),
    }
  }

  pub(crate) fn count(&self) -> usize {
    self.piece_hashes.len()
  }

  pub(crate) fn push(&mut self, digest: Sha1Digest) {
    self.piece_hashes.push(digest);
  }

  #[cfg(test)]
  pub(crate) fn from_pieces<I, B>(pieces: I) -> Self
  where
    I: IntoIterator<Item = B>,
    B: AsRef<[u8]>,
  {
    Self {
      piece_hashes: pieces
        .into_iter()
        .map(|piece| Sha1::from(piece).digest().into())
        .collect(),
    }
  }
}

impl Serialize for PieceList {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut bytes = Vec::with_capacity(self.piece_hashes.len() * sha1::DIGEST_LENGTH);

    for piece in &self.piece_hashes {
      bytes.extend_from_slice(&piece.bytes());
    }

    serde_bytes::Bytes::new(&bytes).serialize(serializer)
  }
}

impl<'de> Deserialize<'de> for PieceList {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let bytes = serde_bytes::ByteBuf::deserialize(deserializer)?.into_vec();

    if bytes.len() % Sha1Digest::LENGTH != 0 {
      return Err(D::Error::custom(format!(
        "buffer length {} is not a multiple of {}",
        bytes.len(),
        sha1::DIGEST_LENGTH
      )));
    }

    let piece_hashes = bytes
      .chunks_exact(Sha1Digest::LENGTH)
      .map(|chunk| {
        Sha1Digest::from_bytes(
          chunk
            .try_into()
            .invariant_unwrap("chunks are all Sha1Digest::LENGTH"),
        )
      })
      .collect();

    Ok(Self { piece_hashes })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn basic() {
    let mut pieces = PieceList::new();
    assert_eq!(pieces.count(), 0);
    pieces.push(Sha1::new().digest().into());
    assert_eq!(pieces.count(), 1);
  }

  fn case(pieces: PieceList, want: impl AsRef<[u8]>) {
    let want = want.as_ref();
    let have = bendy::serde::to_bytes(&pieces).unwrap();
    assert_eq!(
      have,
      want,
      "{} != {}",
      String::from_utf8_lossy(&have),
      String::from_utf8_lossy(want)
    );

    let have = bendy::serde::from_bytes::<PieceList>(want).unwrap();
    assert_eq!(have, pieces);
  }

  #[test]
  fn empty() {
    case(PieceList::new(), "0:");
  }

  #[test]
  fn single() {
    let mut pieces = PieceList::new();
    pieces.push(Sha1::new().digest().into());
    case(
      pieces,
      b"20:\xda\x39\xa3\xee\x5e\x6b\x4b\x0d\x32\x55\xbf\xef\x95\x60\x18\x90\xaf\xd8\x07\x09",
    );
  }
}
