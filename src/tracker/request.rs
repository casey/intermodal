use super::Response;

pub(crate) trait Request {
  type Response: Response;
  fn serialize(&self) -> Vec<u8>;

  fn transaction_id(&self) -> u32;
  fn action(&self) -> u32;
}
