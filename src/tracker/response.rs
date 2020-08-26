use crate::common::*;

pub(crate) trait Response {
  // We leak the response length so that payloads can be parsed. This should be
  // cleaner when associated types within traits can specify named lifetime
  // parameters.
  fn deserialize(buf: &[u8]) -> Result<(Self, usize)>
  where
    Self: std::marker::Sized;

  fn transaction_id(&self) -> u32;
  fn action(&self) -> u32;
}
