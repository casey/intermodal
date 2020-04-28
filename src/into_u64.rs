use crate::common::*;

// Systems with pointers larger than 64 bits may eventually exist, but
// for now let's assume that usize is at most 64 bits, and document that
// assumption with this assert.
const_assert!(std::mem::size_of::<usize>() <= std::mem::size_of::<u64>());

pub(crate) trait IntoU64 {
  fn into_u64(self) -> u64;
}

impl IntoU64 for usize {
  fn into_u64(self) -> u64 {
    #![allow(clippy::as_conversions)]
    self as u64
  }
}
