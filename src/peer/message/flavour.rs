#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Flavour {
  Choke = 0,
  Unchoke,
  Interested,
  NotInterested,
  Have,
  Bitfield,
  Request,
  Piece,
  Cancel,

  HaveAll = 14,
  HaveNone = 15,
  Extended = 20,

  Bad = 255,
}

impl From<u8> for Flavour {
  fn from(i: u8) -> Self {
    match i {
      0x00 => Flavour::Choke,
      0x01 => Flavour::Unchoke,
      0x02 => Flavour::Interested,
      0x03 => Flavour::NotInterested,
      0x04 => Flavour::Have,
      0x05 => Flavour::Bitfield,
      0x06 => Flavour::Request,
      0x07 => Flavour::Piece,
      0x08 => Flavour::Cancel,
      0x0e => Flavour::HaveAll,
      0x0f => Flavour::HaveNone,
      0x14 => Flavour::Extended,
      _ => Flavour::Bad,
    }
  }
}

impl From<Flavour> for u8 {
  fn from(i: Flavour) -> Self {
    match i {
      Flavour::Choke => 0x00,
      Flavour::Unchoke => 0x01,
      Flavour::Interested => 0x02,
      Flavour::NotInterested => 0x03,
      Flavour::Have => 0x04,
      Flavour::Bitfield => 0x05,
      Flavour::Request => 0x06,
      Flavour::Piece => 0x07,
      Flavour::Cancel => 0x08,
      Flavour::HaveAll => 0x0e,
      Flavour::HaveNone => 0x0f,
      Flavour::Extended => 0x14,
      Flavour::Bad => 0xff,
    }
  }
}
