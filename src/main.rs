use crate::common::*;

#[cfg(test)]
#[macro_use]
mod matches;

#[cfg(test)]
mod testing;

mod bencode;
mod common;
mod consts;
mod environment;
mod error;
mod file_info;
mod hasher;
mod info;
mod metainfo;
mod mode;
mod opt;
mod path_ext;
mod reckoner;
mod subcommand;
mod torrent;

fn main() {
  if let Err(code) = Environment::main().status() {
    process::exit(code);
  }
}
