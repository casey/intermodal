// stdlib types
pub(crate) use std::{
  borrow::Cow,
  cmp::{Ordering, Reverse},
  collections::{BTreeMap, BTreeSet, HashMap},
  convert::{Infallible, TryInto},
  env,
  ffi::{OsStr, OsString},
  fmt::{self, Display, Formatter},
  fs::{self, File},
  hash::Hash,
  io::{self, Read, Write},
  num::ParseFloatError,
  ops::{Div, DivAssign, Mul, MulAssign},
  path::{Path, PathBuf},
  process::{self, Command, ExitStatus},
  str::{self, FromStr},
  time::{SystemTime, SystemTimeError},
  usize,
};

// dependencies
pub(crate) use chrono::{TimeZone, Utc};
pub(crate) use libc::EXIT_FAILURE;
pub(crate) use regex::{Regex, RegexSet};
pub(crate) use serde::{Deserialize, Serialize};
pub(crate) use sha1::Sha1;
pub(crate) use snafu::{ResultExt, Snafu};
pub(crate) use static_assertions::const_assert;
pub(crate) use structopt::{
  clap::{AppSettings, ArgSettings},
  StructOpt,
};
pub(crate) use unicode_width::UnicodeWidthStr;
pub(crate) use url::Url;
pub(crate) use walkdir::WalkDir;

// modules
pub(crate) use crate::{bencode, consts, error, use_color};

// traits
pub(crate) use crate::{
  into_u64::IntoU64, into_usize::IntoUsize, path_ext::PathExt,
  platform_interface::PlatformInterface, reckoner::Reckoner,
};

// structs and enums
pub(crate) use crate::{
  bytes::Bytes, env::Env, error::Error, file_info::FileInfo, files::Files, hasher::Hasher,
  info::Info, lint::Lint, linter::Linter, metainfo::Metainfo, mode::Mode, opt::Opt,
  piece_length_picker::PieceLengthPicker, platform::Platform, style::Style, table::Table,
  torrent_summary::TorrentSummary, use_color::UseColor,
};

// test stdlib types
#[cfg(test)]
pub(crate) use std::{
  cell::RefCell,
  io::Cursor,
  ops::{Deref, DerefMut},
  rc::Rc,
  time::{Duration, Instant},
};

// test modules
#[cfg(test)]
pub(crate) use crate::testing;

// test structs and enums
#[cfg(test)]
pub(crate) use crate::{capture::Capture, test_env::TestEnv, test_env_builder::TestEnvBuilder};
