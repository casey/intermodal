use crate::common::*;

// Intermodal, or at least parts of it, might eventually be ported to
// 16-bit systems, but for now let's assume that usize is at least 32
// bits, and document that assumption with this assert.
const_assert!(std::mem::size_of::<usize>() >= std::mem::size_of::<u32>());

pub(crate) trait IntoUsize {
  fn into_usize(self) -> usize;
}

impl IntoUsize for u32 {
  fn into_usize(self) -> usize {
    #![allow(clippy::as_conversions)]
    self as usize
  }
}
