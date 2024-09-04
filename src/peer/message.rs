use crate::common::*;

pub mod extended;
pub mod flavour;

pub(crate) use flavour::Flavour;

#[derive(Debug)]
pub(crate) struct Message {
  pub(crate) flavour: Flavour,
  pub(crate) payload: Option<Vec<u8>>,
}

impl Message {
  pub(crate) fn new(flavour: Flavour, payload: Option<Vec<u8>>) -> Self {
    Message { flavour, payload }
  }

  /// Create a new extended message. Since extended message ids for the same extension protocol may
  /// vary between peers, the id parameter must be determined from an extended handshake or prior
  /// knowledge.
  pub(crate) fn new_extended<T: serde::Serialize>(id: u8, p: T) -> Result<Self> {
    let mut payload = vec![id];
    payload.extend_from_slice(&Self::bencode(p)?);
    Ok(Self::new(Flavour::Extended, Some(payload)))
  }

  #[cfg(test)]
  // Create a new extended message but append `buf` to the bencoded message payload.
  pub(crate) fn new_extended_with_trailer<T: serde::Serialize>(
    id: u8,
    payload: T,
    buf: &[u8],
  ) -> Result<Self> {
    let mut m = Self::new_extended(id, payload)?;
    if let Some(p) = &mut m.payload {
      p.extend_from_slice(buf);
    }
    Ok(m)
  }

  pub(crate) fn len(&self) -> usize {
    1 + self.payload.as_ref().map_or(0, std::vec::Vec::len)
  }

  // Serialize the message to the BitTorrent wire format:
  //
  //   prefix  4 bytes
  //   flavour 1 byte
  //   payload x bytes
  //
  // where prefix is the network byte order encoding of `x + 1` into a u32.
  pub(crate) fn serialize(&self) -> Result<Vec<u8>> {
    // find a way to test this without blowing the stack.
    let message_length = self.len().try_into().context(error::PeerMessagePayload)?;
    let mut buf: Vec<u8> = u32::to_be_bytes(message_length).to_vec();
    buf.push(self.flavour.into());
    if let Some(ref p) = &self.payload {
      buf.extend(p);
    }

    Ok(buf)
  }

  pub(crate) fn parse_extended_payload(&self) -> Result<(extended::Id, &[u8])> {
    if let Some(p) = &self.payload {
      if !p.is_empty() {
        return Ok((p[0].into(), &p[1..]));
      }
    }
    Err(Error::PeerMessageExtendedPayload)
  }

  pub fn bencode<T: serde::Serialize>(msg: T) -> Result<Vec<u8>> {
    bendy::serde::ser::to_bytes(&msg).context(error::PeerMessageBencode)
  }

  pub fn from_bencode<'a, T: serde::Deserialize<'a>>(buf: &'a [u8]) -> Result<T> {
    bendy::serde::de::from_bytes(buf).context(error::PeerMessageFromBencode)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_extended_payload() {
    let m = Message {
      flavour: Flavour::Extended,
      payload: Some(vec![1, 2, 3, 4]),
    };
    let (i, b) = m.parse_extended_payload().unwrap();
    assert_eq!(i, extended::Id::UtMetadata);
    assert_eq!(b, vec![2, 3, 4]);
  }

  #[test]
  fn parse_extended_payload_fail() {
    let m = Message {
      flavour: Flavour::Extended,
      payload: None,
    };
    assert_matches!(
      m.parse_extended_payload(),
      Err(Error::PeerMessageExtendedPayload)
    );
  }
}
