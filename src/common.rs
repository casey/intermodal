// stdlib types
pub(crate) use std::{
  borrow::Cow,
  char,
  cmp::Reverse,
  collections::{BTreeMap, BTreeSet, HashMap},
  convert::{Infallible, TryInto},
  env,
  ffi::{OsStr, OsString},
  fmt::{self, Display, Formatter},
  fs::{self, File},
  hash::Hash,
  io::{self, Read, Write},
  iter::Sum,
  num::{ParseFloatError, ParseIntError, TryFromIntError},
  ops::{AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
  path::{self, Path, PathBuf},
  process::{self, Command, ExitStatus},
  str::{self, FromStr},
  time::{SystemTime, SystemTimeError},
  usize,
};

// dependencies
pub(crate) use bendy::{decoding::FromBencode, encoding::ToBencode, value::Value};
pub(crate) use chrono::{TimeZone, Utc};
pub(crate) use globset::{Glob, GlobMatcher};
pub(crate) use indicatif::{ProgressBar, ProgressStyle};
pub(crate) use libc::EXIT_FAILURE;
pub(crate) use regex::{Regex, RegexSet};
pub(crate) use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};
pub(crate) use serde_hex::SerHex;
pub(crate) use serde_with::rust::unwrap_or_skip;
pub(crate) use sha1::Sha1;
pub(crate) use snafu::{ResultExt, Snafu};
pub(crate) use static_assertions::const_assert;
pub(crate) use structopt::{
  clap::{AppSettings, ArgSettings},
  StructOpt,
};
pub(crate) use unicode_width::UnicodeWidthStr;
pub(crate) use url::{Host, Url};
pub(crate) use walkdir::WalkDir;

// modules
pub(crate) use crate::{consts, error};

// traits
pub(crate) use crate::{
  into_u64::IntoU64, into_usize::IntoUsize, path_ext::PathExt,
  platform_interface::PlatformInterface, print::Print, reckoner::Reckoner, step::Step,
};

// structs and enums
pub(crate) use crate::{
  arguments::Arguments, bytes::Bytes, env::Env, error::Error, file_error::FileError,
  file_info::FileInfo, file_path::FilePath, file_status::FileStatus, files::Files, hasher::Hasher,
  info::Info, lint::Lint, linter::Linter, md5_digest::Md5Digest, metainfo::Metainfo, mode::Mode,
  node::Node, options::Options, output_stream::OutputStream, output_target::OutputTarget,
  piece_length_picker::PieceLengthPicker, piece_list::PieceList, platform::Platform,
  sha1_digest::Sha1Digest, status::Status, style::Style, subcommand::Subcommand, table::Table,
  torrent_summary::TorrentSummary, use_color::UseColor, verifier::Verifier, walker::Walker,
};

// type aliases
pub(crate) type Result<T, E = Error> = std::result::Result<T, E>;

#[cfg(test)]
mod test {
  // test stdlib types
  pub(crate) use std::{
    cell::RefCell,
    io::Cursor,
    ops::{Deref, DerefMut},
    rc::Rc,
    time::{Duration, Instant},
  };

  // test dependencies
  pub(crate) use tempfile::TempDir;
  pub(crate) use temptree::temptree;

  // test structs and enums
  pub(crate) use crate::{capture::Capture, test_env::TestEnv, test_env_builder::TestEnvBuilder};
}

#[cfg(test)]
pub(crate) use test::*;
