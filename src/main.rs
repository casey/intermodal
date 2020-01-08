#![deny(clippy::all, clippy::pedantic, clippy::restriction)]
#![allow(
  clippy::else_if_without_else,
  clippy::enum_glob_use,
  clippy::implicit_return,
  clippy::indexing_slicing,
  clippy::integer_arithmetic,
  clippy::missing_docs_in_private_items,
  clippy::option_map_unwrap_or_else,
  clippy::option_unwrap_used,
  clippy::result_expect_used,
  clippy::result_unwrap_used
)]

use crate::common::*;

#[cfg(test)]
#[macro_use]
mod matches;

#[macro_use]
mod errln;

#[macro_use]
mod err;

#[cfg(test)]
mod testing;

mod bencode;
mod common;
mod consts;
mod env;
mod error;
mod file_info;
mod hasher;
mod info;
mod into_u64;
mod into_usize;
mod metainfo;
mod mode;
mod opt;
mod path_ext;
mod reckoner;
mod subcommand;
mod torrent;

fn main() {
  if let Err(code) = Env::main().status() {
    process::exit(code);
  }
}
