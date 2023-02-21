use crate::common::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Request {
  pub(crate) protocol_id: u64,
  pub(crate) action: u32,
  pub(crate) transaction_id: u32,
}

impl Request {
  pub(crate) const LENGTH: usize = 16;

  const UDP_TRACKER_MAGIC: u64 = 0x0000_0417_2710_1980;

  pub(crate) fn new() -> Self {
    Self {
      protocol_id: Self::UDP_TRACKER_MAGIC,
      action: tracker::Action::Connect.into(),
      transaction_id: rand::thread_rng().gen(),
    }
  }
}

#[derive(Debug, PartialEq)]
pub(crate) struct Response {
  pub(crate) action: u32,
  pub(crate) transaction_id: u32,
  pub(crate) connection_id: u64,
}

impl Response {
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
  fn deserialize(buf: &[u8]) -> Result<(Self, &[u8])> {
    if buf.len() != Self::LENGTH {
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
      &buf[Self::LENGTH..],
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
  fn deserialize(buf: &[u8]) -> Result<(Self, &[u8])> {
    if buf.len() < Self::LENGTH {
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
      &buf[Self::LENGTH..],
    ))
  }

  fn transaction_id(&self) -> u32 {
    self.transaction_id
  }

  fn action(&self) -> u32 {
    self.action
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::tracker::{connect, request::Request, response::Response};

  #[test]
  pub(crate) fn connect_request_roundtrip() {
    let req = connect::Request {
      protocol_id: 0x1337_beef_babe_cafe,
      action: 50,
      transaction_id: 1234,
    };

    let buf = req.serialize();
    let (req2, _) = connect::Request::deserialize(&buf).unwrap();
    assert_eq!(req, req2);
  }

  #[test]
  pub(crate) fn connect_response_roundtrip() {
    let resp = connect::Response {
      action: 50,
      transaction_id: 1234,
      connection_id: 0x1337_beef_babe_cafe,
    };

    let buf = resp.serialize();
    let (resp2, _) = connect::Response::deserialize(&buf).unwrap();
    assert_eq!(resp, resp2);
  }

  #[test]
  pub(crate) fn connect_request_datagram_size() {
    let buf = [0x01, 0x02, 0x03];
    let err = connect::Request::deserialize(&buf);
    assert_matches!(err, Err(Error::TrackerResponse));
  }

  #[test]
  pub(crate) fn connect_response_datagram_size() {
    let buf = [0x01, 0x02, 0x03];
    let err = connect::Response::deserialize(&buf);
    assert_matches!(err, Err(Error::TrackerResponse));
  }
}
