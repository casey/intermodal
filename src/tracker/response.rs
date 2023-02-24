use crate::common::*;

pub(crate) trait Response {
  // Deserialize the response into a Response object and payload.
  fn deserialize(buf: &[u8]) -> Result<(Self, &[u8])>
  where
    Self: std::marker::Sized;

  fn transaction_id(&self) -> u32;
  fn action(&self) -> u32;
}
