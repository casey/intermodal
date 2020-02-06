use crate::common::*;

use structopt::clap;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub(crate) enum Error {
  #[snafu(display("Must provide at least one announce URL"))]
  AnnounceEmpty,
  #[snafu(display("Failed to parse announce URL: {}", source))]
  AnnounceUrlParse { source: url::ParseError },
  #[snafu(display("Failed to deserialize torrent metainfo from `{}`: {}", path.display(), source))]
  MetainfoLoad {
    source: serde_bencode::Error,
    path: PathBuf,
  },
  #[snafu(display("Failed to serialize torrent metainfo: {}", source))]
  MetainfoSerialize { source: serde_bencode::Error },
  #[snafu(display("Failed to parse byte count `{}`: {}", text, source))]
  ByteParse {
    text: String,
    source: ParseFloatError,
  },
  #[snafu(display("Failed to parse byte count `{}`, invalid suffix: `{}`", text, suffix))]
  ByteSuffix { text: String, suffix: String },
  #[snafu(display("{}", source))]
  Clap { source: clap::Error },
  #[snafu(display("Failed to invoke command `{}`: {}", command, source,))]
  CommandInvoke { command: String, source: io::Error },
  #[snafu(display("Command `{}` returned bad exit status: {}", command, status))]
  CommandStatus { command: String, status: ExitStatus },
  #[snafu(display("Filename was not valid unicode: {}", filename.to_string_lossy()))]
  FilenameDecode { filename: OsString },
  #[snafu(display("Path had no file name: {}", path.display()))]
  FilenameExtract { path: PathBuf },
  #[snafu(display("I/O error at `{}`: {}", path.display(), source))]
  Filesystem { source: io::Error, path: PathBuf },
  #[snafu(display("Failed to find opener utility, please install one of {}", tried.join(",")))]
  OpenerMissing { tried: &'static [&'static str] },
  #[snafu(display(
    "Path `{}` contains non-normal component: {}",
    path.display(),
    component.display(),
  ))]
  PathComponent { component: PathBuf, path: PathBuf },
  #[snafu(display(
    "Path `{}` contains non-unicode component: {}",
    path.display(),
    component.display(),
  ))]
  PathDecode { path: PathBuf, component: PathBuf },
  #[snafu(display(
    "Path `{}` empty after stripping prefix `{}`",
    path.display(),
    prefix.display(),
  ))]
  PathStripEmpty { path: PathBuf, prefix: PathBuf },
  #[snafu(display(
    "Failed to strip prefix `{}` from path `{}`: {}",
    prefix.display(),
    path.display(),
    source
  ))]
  PathStripPrefix {
    path: PathBuf,
    prefix: PathBuf,
    source: path::StripPrefixError,
  },
  #[snafu(display(
    "Piece length `{}` too large. The maximum supported piece length is {}.",
    bytes,
    Bytes(u32::max_value().into())
  ))]
  PieceLengthTooLarge { bytes: Bytes },
  #[snafu(display("Piece length `{}` is not an even power of two", bytes))]
  PieceLengthUneven { bytes: Bytes },
  #[snafu(display("Piece length must be at least 16 KiB"))]
  PieceLengthSmall,
  #[snafu(display("Piece length cannot be zero"))]
  PieceLengthZero,
  #[snafu(display("Failed to write to standard error: {}", source))]
  Stderr { source: io::Error },
  #[snafu(display("Failed to write to standard output: {}", source))]
  Stdout { source: io::Error },
  #[snafu(display(
      "Attempted to create torrent from symlink `{}`. To override, pass the `--follow-symlinks` flag.",
      root.display()
  ))]
  SymlinkRoot { root: PathBuf },
  #[snafu(display("Failed to retrieve system time: {}", source))]
  SystemTime { source: SystemTimeError },
  #[snafu(display(
    "Feature `{}` cannot be used without passing the `--unstable` flag",
    feature
  ))]
  Unstable { feature: &'static str },
  #[snafu(display("Unknown lint: {}", text))]
  LintUnknown { text: String },
}

impl Error {
  pub(crate) fn lint(&self) -> Option<Lint> {
    match self {
      Self::PieceLengthUneven { .. } => Some(Lint::UnevenPieceLength),
      Self::PieceLengthSmall { .. } => Some(Lint::SmallPieceLength),
      _ => None,
    }
  }
}

impl From<clap::Error> for Error {
  fn from(source: clap::Error) -> Self {
    Self::Clap { source }
  }
}

impl From<SystemTimeError> for Error {
  fn from(source: SystemTimeError) -> Self {
    Self::SystemTime { source }
  }
}

impl From<walkdir::Error> for Error {
  fn from(walkdir_error: walkdir::Error) -> Self {
    let path = walkdir_error.path().unwrap().to_owned();

    if let Some(source) = walkdir_error.into_io_error() {
      Self::Filesystem { source, path }
    } else {
      unreachable!("Encountered unexpected walkdir error")
    }
  }
}
