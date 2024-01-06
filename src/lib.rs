#![deny(clippy::all, clippy::pedantic)]
#![allow(
  clippy::float_cmp,
  clippy::needless_lifetimes,
  clippy::needless_pass_by_value,
  clippy::non_ascii_literal,
  clippy::struct_excessive_bools,
  clippy::too_many_lines,
  clippy::unseparated_literal_suffix,
  clippy::wildcard_imports,
  clippy::large_enum_variant,
  clippy::module_name_repetitions
)]

pub use run::run;

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
pub mod bench;
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
mod magnet_link_parse_error;
mod md5_digest;
mod metainfo;
mod metainfo_error;
mod mode;
mod options;
mod output_stream;
mod output_target;
mod peer;
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
mod tracker;
mod use_color;
mod verifier;
mod walker;
mod xor_args;
