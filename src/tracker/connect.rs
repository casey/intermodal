use crate::common::*;

#[derive(Clone, Copy, Debug)]
pub(crate) struct Request {
  pub(crate) protocol_id: u64,
  pub(crate) action: u32,
  pub(crate) transaction_id: u32,
}

impl Request {
  #[allow(unused)]
  pub(crate) const LENGTH: usize = 16;
}

#[derive(Debug)]
pub(crate) struct Response {
  pub(crate) action: u32,
  pub(crate) transaction_id: u32,
  pub(crate) connection_id: u64,
}

impl Response {
  #[allow(unused)]
  pub(crate) const LENGTH: usize = 16;
}

impl super::Request for Request {
  type Response = Response;

  fn serialize(&self) -> Vec<u8> {
    let mut msg = Vec::new();

    msg.extend_from_slice(&self.protocol_id.to_be_bytes());
    msg.extend_from_slice(&self.action.to_be_bytes());
    msg.extend_from_slice(&self.transaction_id.to_be_bytes());

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
    if buf.len() != 8 + 4 + 4 {
      return Err(Error::TrackerResponse);
    }

    Ok((
      Request {
        protocol_id: u64::from_be_bytes(
          buf[0..8]
            .try_into()
            .invariant_unwrap("incoming type guarantees bounds are OK"),
        ),
        action: u32::from_be_bytes(
          buf[8..12]
            .try_into()
            .invariant_unwrap("incoming type guarantees bounds are OK"),
        ),
        transaction_id: u32::from_be_bytes(
          buf[12..16]
            .try_into()
            .invariant_unwrap("incoming type guarantees bounds are OK"),
        ),
      },
      16,
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
    msg.extend_from_slice(&self.connection_id.to_be_bytes());

    msg
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
    if buf.len() < 4 + 4 + 8 {
      return Err(Error::TrackerResponse);
    }

    Ok((
      Self {
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
        connection_id: u64::from_be_bytes(
          buf[8..16]
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
