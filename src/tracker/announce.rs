use crate::common::*;

#[derive(Debug)]
pub(crate) struct Request {
  pub(crate) connection_id: u64,  //  8
  pub(crate) action: u32,         // 12
  pub(crate) transaction_id: u32, // 16
  pub(crate) infohash: [u8; 20],  // 36
  pub(crate) peer_id: [u8; 20],   // 56
  pub(crate) downloaded: u64,     // 64
  pub(crate) left: u64,           // 72
  pub(crate) uploaded: u64,       // 80
  pub(crate) event: u64,          // 88
  pub(crate) ip_address: u32,     // 92
  pub(crate) num_want: u32,       // 96
  pub(crate) port: u16,           // 98
}

impl Request {
  #[allow(unused)]
  pub(crate) const LENGTH: usize = 98;
}

#[derive(Debug)]
pub(crate) struct Response {
  pub(crate) action: u32,
  pub(crate) transaction_id: u32,
  pub(crate) interval: u32,
  pub(crate) leechers: u32,
  pub(crate) seeders: u32,
}

impl Response {
  pub(crate) const LENGTH: usize = 20;
}

impl super::Request for Request {
  type Response = Response;

  fn serialize(&self) -> Vec<u8> {
    let mut msg = Vec::new();

    msg.extend_from_slice(&self.connection_id.to_be_bytes());
    msg.extend_from_slice(&self.action.to_be_bytes());
    msg.extend_from_slice(&self.transaction_id.to_be_bytes());
    msg.extend_from_slice(&self.infohash);
    msg.extend_from_slice(&self.peer_id);
    msg.extend_from_slice(&self.downloaded.to_be_bytes());
    msg.extend_from_slice(&self.left.to_be_bytes());
    msg.extend_from_slice(&self.uploaded.to_be_bytes());
    msg.extend_from_slice(&self.event.to_be_bytes());
    msg.extend_from_slice(&self.ip_address.to_be_bytes());
    msg.extend_from_slice(&self.num_want.to_be_bytes());
    msg.extend_from_slice(&self.port.to_be_bytes());

    msg
  }

  fn transaction_id(&self) -> u32 {
    self.transaction_id
  }

  fn action(&self) -> u32 {
    self.action
  }
}

impl super::Response for Request {
  #[allow(dead_code)]
  fn deserialize(buf: &[u8]) -> Result<(Self, usize)> {
    if buf.len() < 98 {
      return Err(Error::TrackerResponse);
    }

    Ok((
      Request {
        connection_id: u64::from_be_bytes(
          buf[0..8]
            .try_into()
            .invariant_unwrap("bounds guaranteed OK by type system"),
        ),
        action: u32::from_be_bytes(
          buf[8..12]
            .try_into()
            .invariant_unwrap("bounds guaranteed OK by type system"),
        ),
        transaction_id: u32::from_be_bytes(
          buf[12..16]
            .try_into()
            .invariant_unwrap("bounds guaranteed OK by type system"),
        ),
        infohash: buf[16..36]
          .try_into()
          .invariant_unwrap("bounds guaranteed OK by type system"),
        peer_id: buf[36..56]
          .try_into()
          .invariant_unwrap("bounds guaranteed OK by type system"),
        downloaded: u64::from_be_bytes(
          buf[56..64]
            .try_into()
            .invariant_unwrap("bounds guaranteed OK by type system"),
        ),
        left: u64::from_be_bytes(
          buf[64..72]
            .try_into()
            .invariant_unwrap("bounds guaranteed OK by type system"),
        ),
        uploaded: u64::from_be_bytes(
          buf[72..80]
            .try_into()
            .invariant_unwrap("bounds guaranteed OK by type system"),
        ),
        event: u64::from_be_bytes(
          buf[80..88]
            .try_into()
            .invariant_unwrap("bounds guaranteed OK by type system"),
        ),
        ip_address: u32::from_be_bytes(
          buf[88..92]
            .try_into()
            .invariant_unwrap("bounds guaranteed OK by type system"),
        ),
        num_want: u32::from_be_bytes(
          buf[92..96]
            .try_into()
            .invariant_unwrap("bounds guaranteed OK by type system"),
        ),
        port: u16::from_be_bytes(
          buf[96..98]
            .try_into()
            .invariant_unwrap("bounds guaranteed OK by type system"),
        ),
      },
      buf.len(),
    ))
  }

  fn transaction_id(&self) -> u32 {
    self.transaction_id
  }

  fn action(&self) -> u32 {
    self.action
  }
}

impl super::Response for Response {
  fn deserialize(buf: &[u8]) -> Result<(Self, usize)> {
    if buf.len() < 4 + 4 + 4 + 4 + 4 {
      return Err(Error::TrackerResponse);
    }

    Ok((
      Response {
        action: u32::from_be_bytes(
          buf[0..4]
            .try_into()
            .invariant_unwrap("bounds are checked manually above"),
        ),
        transaction_id: u32::from_be_bytes(
          buf[4..8]
            .try_into()
            .invariant_unwrap("bounds are checked manually above"),
        ),
        interval: u32::from_be_bytes(
          buf[8..12]
            .try_into()
            .invariant_unwrap("bounds are checked manually above"),
        ),
        leechers: u32::from_be_bytes(
          buf[12..16]
            .try_into()
            .invariant_unwrap("bounds are checked manually above"),
        ),
        seeders: u32::from_be_bytes(
          buf[16..20]
            .try_into()
            .invariant_unwrap("bounds are checked manually above"),
        ),
      },
      buf.len(),
    ))
  }

  fn transaction_id(&self) -> u32 {
    self.transaction_id
  }

  fn action(&self) -> u32 {
    self.action
  }
}

impl super::Request for Response {
  type Response = Request;

  #[allow(dead_code)]
  fn serialize(&self) -> Vec<u8> {
    let mut msg = Vec::new();

    msg.extend_from_slice(&self.action.to_be_bytes());
    msg.extend_from_slice(&self.transaction_id.to_be_bytes());
    msg.extend_from_slice(&self.interval.to_be_bytes());
    msg.extend_from_slice(&self.leechers.to_be_bytes());
    msg.extend_from_slice(&self.seeders.to_be_bytes());

    msg
  }

  fn transaction_id(&self) -> u32 {
    self.transaction_id
  }

  fn action(&self) -> u32 {
    self.action
  }
}
