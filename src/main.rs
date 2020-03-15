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
  clippy::non_ascii_literal,
  clippy::option_map_unwrap_or_else,
  clippy::option_unwrap_used,
  clippy::result_expect_used,
  clippy::result_unwrap_used,
  clippy::shadow_reuse,
  clippy::too_many_arguments,
  clippy::too_many_lines,
  clippy::unreachable,
  clippy::unseparated_literal_suffix,
  clippy::wildcard_enum_match_arm
)]

use crate::common::*;

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
#[macro_use]
mod test_env;

#[cfg(test)]
mod test_env_builder;

#[cfg(test)]
mod capture;

mod arguments;
mod bytes;
mod common;
mod consts;
mod env;
mod error;
mod file_error;
mod file_info;
mod file_path;
mod file_status;
mod files;
mod hasher;
mod info;
mod into_u64;
mod into_usize;
mod lint;
mod linter;
mod md5_digest;
mod metainfo;
mod mode;
mod node;
mod options;
mod output_stream;
mod output_target;
mod path_ext;
mod piece_length_picker;
mod piece_list;
mod platform;
mod platform_interface;
mod print;
mod reckoner;
mod sha1_digest;
mod status;
mod step;
mod style;
mod subcommand;
mod table;
mod torrent_summary;
mod use_color;
mod verifier;
mod walker;

fn main() {
  if let Err(code) = Env::main().status() {
    process::exit(code);
  }
}
