// stdlib types
pub(crate) use std::{
  borrow::Cow,
  cmp::{Ordering, Reverse},
  collections::{BTreeMap, HashMap},
  convert::TryInto,
  env,
  ffi::{OsStr, OsString},
  fmt::{self, Display, Formatter},
  fs::{self, File},
  hash::Hash,
  io::{self, Read, Write},
  path::{Path, PathBuf},
  process, str,
  time::{SystemTime, SystemTimeError},
  usize,
};

// dependencies
pub(crate) use libc::EXIT_FAILURE;
pub(crate) use regex::{Regex, RegexSet};
pub(crate) use serde::{Deserialize, Serialize};
pub(crate) use sha1::Sha1;
pub(crate) use snafu::{ResultExt, Snafu};
pub(crate) use static_assertions::const_assert;
pub(crate) use structopt::StructOpt;
pub(crate) use url::Url;
pub(crate) use walkdir::WalkDir;

// modules
pub(crate) use crate::{bencode, consts, error, torrent};

// traits
pub(crate) use crate::{
  into_u64::IntoU64, into_usize::IntoUsize, path_ext::PathExt, reckoner::Reckoner,
};

// structs and enums
pub(crate) use crate::{
  environment::Environment, error::Error, file_info::FileInfo, hasher::Hasher, info::Info,
  metainfo::Metainfo, mode::Mode, opt::Opt, subcommand::Subcommand, torrent::Torrent,
};

// test modules
#[cfg(test)]
pub(crate) use crate::testing;
