#![no_std]

mod common;
mod data;
mod error;
mod u16;
mod u8;

#[cfg(test)]
mod buffer;

#[cfg(test)]
mod data_test;

pub use data_macros::table;

pub use crate::{data::Data, error::Error};
