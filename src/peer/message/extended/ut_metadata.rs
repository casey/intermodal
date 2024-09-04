use crate::common::*;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub(crate) struct UtMetadata {
  pub(crate) msg_type: u8,
  pub(crate) piece: usize,
  #[serde(
    skip_serializing_if = "Option::is_none",
    default,
    with = "unwrap_or_skip"
  )]
  pub(crate) total_size: Option<usize>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum MsgType {
  Request,
  Data,
  Reject,
}

impl UtMetadata {
  pub(crate) const NAME: &'static str = "ut_metadata";
  pub(crate) const PIECE_LENGTH: usize = 16 * (1 << 10);

  pub(crate) fn request(piece: usize) -> Self {
    Self {
      msg_type: MsgType::Request.into(),
      piece,
      total_size: None,
    }
  }

  #[cfg(test)]
  pub(crate) fn data(piece: usize, total_size: usize) -> Self {
    Self {
      msg_type: MsgType::Data.into(),
      piece,
      total_size: Some(total_size),
    }
  }
}

impl From<MsgType> for u8 {
  fn from(m: MsgType) -> u8 {
    match m {
      MsgType::Request => 0,
      MsgType::Data => 1,
      MsgType::Reject => 2,
    }
  }
}

impl From<u8> for MsgType {
  fn from(x: u8) -> Self {
    match x {
      0 => Self::Request,
      1 => Self::Data,
      _ => MsgType::Reject,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn bencode_extended_metadata_message() {
    let req = UtMetadata {
      msg_type: MsgType::Data.into(),
      piece: 1,
      total_size: None,
    };
    let mut msg = bendy::serde::ser::to_bytes(&req).unwrap();
    let benc = b"d8:msg_typei1e5:piecei1ee";
    assert_eq!(benc, &*msg);
    msg.extend_from_slice(b"piece data goes here...");
    let req2 = bendy::serde::de::from_bytes(&msg).unwrap();
    assert_eq!(req, req2);
  }
}
