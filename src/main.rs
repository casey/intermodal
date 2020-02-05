#![deny(clippy::all, clippy::pedantic, clippy::restriction)]
#![allow(
  clippy::else_if_without_else,
  clippy::enum_glob_use,
  clippy::float_arithmetic,
  clippy::float_cmp,
  clippy::implicit_return,
  clippy::indexing_slicing,
  clippy::integer_arithmetic,
  clippy::integer_division,
  clippy::large_enum_variant,
  clippy::missing_docs_in_private_items,
  clippy::needless_pass_by_value,
  clippy::option_map_unwrap_or_else,
  clippy::option_unwrap_used,
  clippy::result_expect_used,
  clippy::result_unwrap_used,
  clippy::shadow_reuse,
  clippy::unreachable,
  clippy::unseparated_literal_suffix,
  clippy::wildcard_enum_match_arm
)]

use crate::common::*;

#[cfg(test)]
#[macro_use]
mod matches;

#[cfg(test)]
#[macro_use]
mod assert_matches;

#[macro_use]
mod errln;

#[macro_use]
mod err;

#[macro_use]
mod outln;

#[cfg(test)]
mod testing;

#[cfg(test)]
mod test_env;

#[cfg(test)]
mod test_env_builder;

#[cfg(test)]
mod capture;

mod bencode;
mod bytes;
mod common;
mod consts;
mod env;
mod error;
mod file_info;
mod files;
mod hasher;
mod info;
mod into_u64;
mod into_usize;
mod lint;
mod linter;
mod metainfo;
mod mode;
mod opt;
mod path_ext;
mod piece_length_picker;
mod platform;
mod platform_interface;
mod reckoner;
mod style;
mod table;
mod target;
mod torrent_summary;
mod use_color;

fn main() {
  if let Err(code) = Env::main().status() {
    process::exit(code);
  }
}
