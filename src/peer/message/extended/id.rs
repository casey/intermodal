use super::*;

#[derive(Debug, Eq, Hash, PartialEq)]
pub(crate) enum Id {
  Handshake,
  UtMetadata,
  NotImplemented(u8),
}

impl From<Id> for String {
  fn from(i: Id) -> Self {
    match i {
      Id::Handshake => "handshake".to_string(),
      Id::UtMetadata => "ut_metadata".to_string(),
      Id::NotImplemented(_) => "not supported".to_string(),
    }
  }
}

impl From<u8> for Id {
  fn from(ins: u8) -> Self {
    match ins {
      0x00 => Id::Handshake,
      0x01 => Id::UtMetadata,
      _ => Id::NotImplemented(ins),
    }
  }
}

impl From<Id> for u8 {
  fn from(ins: Id) -> Self {
    match ins {
      Id::Handshake => 0x00,
      Id::UtMetadata => 0x01,
      Id::NotImplemented(n) => n,
    }
  }
}
