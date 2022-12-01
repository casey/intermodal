use crate::common::*;

pub(crate) const HEADER: &[u8; 20] = b"\x13BitTorrent protocol";
pub(crate) const SUPPORTS_EXTENSION_PROTOCOL: u8 = 0b0001_0000;
pub(crate) const IMDL_RESERVED_BYTES: [u8; 8] = [0, 0, 0, 0, 0, SUPPORTS_EXTENSION_PROTOCOL, 0, 0];

#[derive(Debug)]
pub(crate) struct Handshake {
  pub(crate) peer_id: [u8; 20],
  pub(crate) infohash: [u8; 20],
  pub(crate) reserved: [u8; 8],
}

impl Handshake {
  pub(crate) const LENGTH: usize = 68;

  pub(crate) fn new(infohash: Infohash) -> Self {
    Handshake {
      peer_id: rand::thread_rng().gen(),
      infohash: infohash.into(),
      reserved: IMDL_RESERVED_BYTES,
    }
  }

  pub(crate) fn serialize(&self) -> [u8; Handshake::LENGTH] {
    let mut msg = [0u8; Handshake::LENGTH];

    msg[0..20].copy_from_slice(HEADER);
    msg[20..28].copy_from_slice(&self.reserved);
    msg[28..48].copy_from_slice(&self.infohash);
    msg[48..68].copy_from_slice(&self.peer_id);

    msg
  }

  pub fn supports_extension_protocol(&self) -> bool {
    self.reserved[5] & SUPPORTS_EXTENSION_PROTOCOL > 0
  }
}

impl TryFrom<[u8; Handshake::LENGTH]> for Handshake {
  type Error = Error;

  fn try_from(buf: [u8; Handshake::LENGTH]) -> Result<Self> {
    if &buf[0..20] != HEADER {
      return Err(error::Error::PeerHandshakeHeader);
    }

    let mut infohash = [0u8; 20];
    let mut reserved = [0u8; 8];
    let mut peer_id = [0u8; 20];

    reserved.clone_from_slice(&buf[20..28]);
    infohash.clone_from_slice(&buf[28..48]);
    peer_id.clone_from_slice(&buf[48..68]);

    Ok(Handshake {
      peer_id,
      infohash,
      reserved,
    })
  }
}
