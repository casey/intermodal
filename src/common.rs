// stdlib types
pub(crate) use std::{
  borrow::Cow,
  char,
  cmp::{Ordering, Reverse},
  collections::{BTreeMap, BTreeSet, HashMap, HashSet},
  convert::{TryFrom, TryInto},
  env,
  ffi::{OsStr, OsString},
  fmt::{self, Display, Formatter},
  fs::{self, File},
  hash::Hash,
  io::{self, BufRead, BufReader, Cursor, Read, Write},
  iter::{self, Sum},
  net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, TcpStream, ToSocketAddrs, UdpSocket},
  num::{ParseFloatError, ParseIntError, TryFromIntError},
  ops::{AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
  path::{self, Path, PathBuf},
  str::{self, FromStr},
  string::FromUtf8Error,
  sync::{mpsc::channel, Once},
  time::{Duration, SystemTime, SystemTimeError},
};

// dependencies
pub(crate) use bendy::{decoding::FromBencode, encoding::ToBencode, value::Value};
pub(crate) use chrono::{TimeZone, Utc};
pub(crate) use globset::{Glob, GlobMatcher};
pub(crate) use ignore::WalkBuilder;
pub(crate) use indicatif::{ProgressBar, ProgressStyle};
pub(crate) use lexiclean::Lexiclean;
pub(crate) use libc::EXIT_FAILURE;
pub(crate) use rand::Rng;
pub(crate) use regex::{Regex, RegexSet};
pub(crate) use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};
pub(crate) use serde_hex::SerHex;
pub(crate) use serde_with::rust::unwrap_or_skip;
pub(crate) use sha1::Sha1;
pub(crate) use snafu::{ResultExt, Snafu};
pub(crate) use static_assertions::const_assert;
pub(crate) use structopt::{
  clap::{self, AppSettings},
  StructOpt,
};
pub(crate) use strum::{IntoEnumIterator, VariantNames};
pub(crate) use strum_macros::{EnumIter, EnumString, EnumVariantNames, IntoStaticStr};
pub(crate) use unicode_width::UnicodeWidthStr;
pub(crate) use url::{Host, Url};

// logging functions
#[allow(unused_imports)]
pub(crate) use log::trace;

// modules
pub(crate) use crate::{
  consts, error, host_port_parse_error, magnet_link_parse_error, peer, tracker,
};

// functions
pub(crate) use crate::xor_args::xor_args;

// traits
pub(crate) use crate::{
  input_stream::InputStream, into_u64::IntoU64, into_usize::IntoUsize, invariant::Invariant,
  platform_interface::PlatformInterface, print::Print, reckoner::Reckoner, step::Step,
};

// structs and enums
pub(crate) use crate::{
  arguments::Arguments, bytes::Bytes, env::Env, error::Error, file_error::FileError,
  file_info::FileInfo, file_path::FilePath, file_status::FileStatus, files::Files, hasher::Hasher,
  host_port::HostPort, host_port_parse_error::HostPortParseError, info::Info, infohash::Infohash,
  input::Input, input_target::InputTarget, lint::Lint, linter::Linter, magnet_link::MagnetLink,
  magnet_link_parse_error::MagnetLinkParseError, md5_digest::Md5Digest, metainfo::Metainfo,
  metainfo_error::MetainfoError, mode::Mode, options::Options, output_stream::OutputStream,
  output_target::OutputTarget, piece_length_picker::PieceLengthPicker, piece_list::PieceList,
  platform::Platform, sha1_digest::Sha1Digest, shell::Shell, sort_key::SortKey,
  sort_order::SortOrder, sort_spec::SortSpec, status::Status, style::Style, subcommand::Subcommand,
  table::Table, torrent_summary::TorrentSummary, use_color::UseColor, verifier::Verifier,
  walker::Walker,
};

// type aliases
pub(crate) type Result<T, E = Error> = std::result::Result<T, E>;

#[cfg(test)]
pub(crate) use test::*;

#[cfg(test)]
mod test {
  // test stdlib types
  pub(crate) use std::{
    cell::RefCell,
    net::TcpListener,
    ops::{Deref, DerefMut},
    process::Command,
    rc::Rc,
    thread,
  };

  // test dependencies
  pub(crate) use tempfile::TempDir;
  pub(crate) use temptree::temptree;

  // test structs and enums
  pub(crate) use crate::{capture::Capture, test_env::TestEnv, test_env_builder::TestEnvBuilder};
}
