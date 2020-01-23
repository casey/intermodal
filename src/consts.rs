/// Default value for `created by` torrent metainfo field.
///
/// Example: imdl/0.0.0 (1234567890AB)
pub(crate) const CREATED_BY_DEFAULT: &str = concat!(
  "imdl/",
  env!("CARGO_PKG_VERSION"),
  " (",
  env!("GIT_HEAD_PARTIAL_HASH"),
  ")"
);

/// Value for `encoding` torrent metainfo field.
pub(crate) const ENCODING_UTF8: &str = "UTF-8";

pub(crate) const ABOUT: &str = concat!(
  env!("CARGO_PKG_DESCRIPTION"),
  " - ",
  env!("CARGO_PKG_HOMEPAGE")
);

pub(crate) const VERSION: &str = concat!("v", env!("CARGO_PKG_VERSION"));

pub(crate) const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");

pub(crate) const HELP_MESSAGE: &str = "Print help message";

pub(crate) const VERSION_MESSAGE: &str = "Print version number";
