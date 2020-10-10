#![deny(clippy::all, clippy::pedantic, clippy::restriction)]
#![allow(
  clippy::blanket_clippy_restriction_lints,
  clippy::else_if_without_else,
  clippy::enum_glob_use,
  clippy::float_arithmetic,
  clippy::float_cmp,
  clippy::implicit_return,
  clippy::indexing_slicing,
  clippy::integer_arithmetic,
  clippy::integer_division,
  clippy::large_enum_variant,
  clippy::map_unwrap_or,
  clippy::missing_docs_in_private_items,
  clippy::missing_inline_in_public_items,
  clippy::needless_pass_by_value,
  clippy::non_ascii_literal,
  clippy::pattern_type_mismatch,
  clippy::shadow_reuse,
  clippy::struct_excessive_bools,
  clippy::too_many_lines,
  clippy::unseparated_literal_suffix,
  clippy::wildcard_enum_match_arm,
  clippy::wildcard_imports
)]

#[cfg(test)]
#[macro_use]
mod assert_matches;

#[macro_use]
mod errln;

#[macro_use]
mod err;

#[macro_use]
mod out;

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
mod host_port;
mod host_port_parse_error;
mod info;
mod infohash;
mod input;
mod input_stream;
mod input_target;
mod into_u64;
mod into_usize;
mod invariant;
mod lint;
mod linter;
mod magnet_link;
mod md5_digest;
mod metainfo;
mod metainfo_error;
mod mode;
mod options;
mod output_stream;
mod output_target;
mod piece_length_picker;
mod piece_list;
mod platform;
mod platform_interface;
mod print;
mod reckoner;
mod run;
mod sha1_digest;
mod shell;
mod sort_key;
mod sort_order;
mod sort_spec;
mod status;
mod step;
mod style;
mod subcommand;
mod table;
mod torrent_summary;
mod use_color;
mod verifier;
mod walker;
mod xor_args;

#[cfg(feature = "bench")]
pub mod bench;

pub use run::run;
