pub(crate) use core::{result, str::Utf8Error};

pub(crate) use crate::error::Error;

pub(crate) use crate::data::Data;

pub(crate) type Result<T, E = Error> = result::Result<T, E>;

#[cfg(test)]
mod test {
  pub(crate) extern crate alloc;

  pub(crate) use core::{
    fmt::Debug,
    ops::{Deref, DerefMut},
  };

  pub(crate) use alloc::{alloc::Layout, vec::Vec};

  pub(crate) use crate::buffer::Buffer;

  pub(crate) use crate::data_test::DataTest;
}

#[cfg(test)]
pub(crate) use test::*;
