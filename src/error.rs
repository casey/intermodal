use crate::common::*;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub(crate) enum Error {
  #[snafu(display("Failed to parse announce URL: {}", source))]
  AnnounceUrlParse { source: url::ParseError },
  #[snafu(display("Failed to parse byte count `{}`: {}", text, source))]
  ByteParse {
    text: String,
    source: ParseFloatError,
  },
  #[snafu(display("Failed to parse byte count `{}`, invalid suffix: `{}`", text, suffix))]
  ByteSuffix { text: String, suffix: String },
  #[snafu(display("{}", source))]
  Clap { source: clap::Error },
  #[snafu(display("Failed to get current directory: {}", source))]
  CurrentDirectoryGet { source: io::Error },
  #[snafu(display("Filename was not valid unicode: `{}`", filename.display()))]
  FilenameDecode { filename: PathBuf },
  #[snafu(display("Path had no file name: `{}`", path.display()))]
  FilenameExtract { path: PathBuf },
  #[snafu(display("I/O error at `{}`: {}", path.display(), source))]
  Filesystem { source: io::Error, path: PathBuf },
  #[snafu(display("Error searching for files: {}", source))]
  FileSearch { source: ignore::Error },
  #[snafu(display("Failed to fetch infodict from accessible peers"))]
  FromLinkNoInfo,
  #[snafu(display("Invalid glob: {}", source))]
  GlobParse { source: globset::Error },
  #[snafu(display("Failed to serialize torrent info dictionary: {}", source))]
  InfoSerialize { source: bendy::serde::Error },
  #[snafu(display("Input target empty"))]
  InputTargetEmpty,
  #[snafu(display(
    "Interal error, this may indicate a bug in intermodal: {}\n\
     Consider filing an issue: https://github.com/casey/imdl/issues/new",
    message,
  ))]
  Internal { message: String },
  #[snafu(display("Failed to parse magnet link `{}`: {}", text, source))]
  MagnetLinkParse {
    text: String,
    source: MagnetLinkParseError,
  },
  #[snafu(display("Failed to deserialize torrent metainfo from {}: {}", input, source))]
  MetainfoDeserialize {
    source: bendy::serde::Error,
    input: InputTarget,
  },
  #[snafu(display("Torrent metainfo does not specify any usable trackers"))]
  MetainfoMissingTrackers,
  #[snafu(display("Failed to serialize torrent metainfo: {}", source))]
  MetainfoSerialize { source: bendy::serde::Error },
  #[snafu(display("Failed to decode metainfo bencode from {}: {}", input, error))]
  MetainfoDecode {
    input: InputTarget,
    error: bendy::decoding::Error,
  },
  #[snafu(display("Metainfo from {} failed to validate: {}", input, source))]
  MetainfoValidate {
    input: InputTarget,
    source: MetainfoError,
  },
  #[snafu(display("Network error: {}", source))]
  Network { source: io::Error },
  #[snafu(display("Failed to invoke opener: {}", source))]
  OpenerInvoke { source: io::Error },
  #[snafu(display("Output path already exists: `{}`", path.display()))]
  OutputExists { path: PathBuf },
  #[snafu(display("Output target empty"))]
  OutputTargetEmpty,
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
    Bytes(u32::MAX.into())
  ))]
  PieceLengthTooLarge {
    bytes: Bytes,
    source: TryFromIntError,
  },
  #[snafu(display("Received peer handshake with the wrong infohash"))]
  PeerHandshakeInfohash,
  #[snafu(display("Received peer handshake with the wrong protocol header"))]
  PeerHandshakeHeader,
  #[snafu(display("Bencoding error: `{}`", source))]
  PeerMessageBencode { source: bendy::serde::Error },
  #[snafu(display("Peer extended message payload is malformed"))]
  PeerMessageExtendedPayload,
  #[snafu(display("Failed to decode bencoded message: `{}`", source))]
  PeerMessageFromBencode { source: bendy::serde::Error },
  #[snafu(display("Peer message payload is too large"))]
  PeerMessagePayload { source: TryFromIntError },
  #[snafu(display("Extended handshake has not been received from peer"))]
  PeerNoExtendedHandshake,
  #[snafu(display("Received UtMetadata info dict that's failed to deserialize"))]
  PeerUtMetadataInfoDeserialize { source: bendy::serde::Error },
  #[snafu(display("Received UtMetadata info dict that's too long"))]
  PeerUtMetadataInfoLength,
  #[snafu(display("Received UtMetadata data message that's too long"))]
  PeerUtMetadataPieceLength,
  #[snafu(display("Peer doesn't know metadata size"))]
  PeerUtMetadataMetadataSizeNotKnown,
  #[snafu(display("Peer doesn't support UtMetadata extension"))]
  PeerUtMetadataNotSupported,
  #[snafu(display("Hash of received info dict does not match"))]
  PeerUtMetadataWrongInfohash,
  #[snafu(display("Received the wrong UtMetadata piece"))]
  PeerUtMetadataWrongPiece,
  #[snafu(display("Piece length `{}` is not an even power of two", bytes))]
  PieceLengthUneven { bytes: Bytes },
  #[snafu(display("Piece length must be at least 16 KiB"))]
  PieceLengthSmall,
  #[snafu(display("Piece length cannot be zero"))]
  PieceLengthZero,
  #[snafu(display("Private torrents must have tracker"))]
  PrivateTrackerless,
  #[snafu(display("Completion script for shell `{}` not UTF-8: {}", shell.name(), source))]
  ShellDecode { shell: Shell, source: FromUtf8Error },
  #[snafu(display("Failed to write to standard error: {}", source))]
  Stderr { source: io::Error },
  #[snafu(display("Failed to read from standard input: {}", source))]
  Stdin { source: io::Error },
  #[snafu(display("Failed to write to standard output: {}", source))]
  Stdout { source: io::Error },
  #[snafu(display(
      "Attempted to create torrent from symlink `{}`. To override, pass the \
      `--follow-symlinks` flag.",
      root.display()
  ))]
  SymlinkRoot { root: PathBuf },
  #[snafu(display("Failed to retrieve system time: {}", source))]
  SystemTime { source: SystemTimeError },
  #[snafu(display("Compact peer list is not the expected length"))]
  TrackerCompactPeerList,
  #[snafu(display("Tracker exchange to `udp://{}` timed out.", tracker_addr))]
  TrackerExchange { tracker_addr: SocketAddr },
  #[snafu(display(
    "Cannot connect to tracker `{}`: URL does not specify a valid host port",
    tracker_url
  ))]
  TrackerHostPort {
    source: HostPortParseError,
    tracker_url: Url,
  },
  #[snafu(display("Tracker client cannot announce without a connection id"))]
  TrackerNoConnectionId,
  #[snafu(display("Tracker resolved to no useable addresses"))]
  TrackerNoHosts,
  #[snafu(display("Malformed response from tracker"))]
  TrackerResponse,
  #[snafu(display("Response from tracker has wrong length: got {}; want {}", got, want))]
  TrackerResponseLength { want: usize, got: usize },
  #[snafu(display("Tracker failed to send datagram: {}", source))]
  TrackerSend { source: io::Error },
  #[snafu(display("Failed to resolve socket addrs: {}", source))]
  TrackerSocketAddrs { source: io::Error },
  #[snafu(display(
    "Cannot connect to tracker `{}`: only UDP trackers are supported",
    tracker_url
  ))]
  TrackerUdpOnly { tracker_url: Url },
  #[snafu(display("Failed to bind to UDP socket: {}", source))]
  UdpSocketBind { source: io::Error },
  #[snafu(display("Failed to connect to `udp://{}`: {}", addr, source))]
  UdpSocketConnect { addr: SocketAddr, source: io::Error },
  #[snafu(display("Failed to get local UDP socket address: {}", source))]
  UdpSocketLocalAddress { source: io::Error },
  #[snafu(display("Failed to set read timeout: {}", source))]
  UdpSocketReadTimeout { source: io::Error },
  #[snafu(display(
    "Feature `{}` cannot be used without passing the `--unstable` flag",
    feature
  ))]
  Unstable { feature: &'static str },
  #[snafu(display("Torrent verification failed."))]
  Verify,
  #[snafu(display("Failed to serialize JSON: {}", source))]
  JsonSerialize { source: serde_json::Error },
}

impl Error {
  pub(crate) fn lint(&self) -> Option<Lint> {
    match self {
      Self::PieceLengthUneven { .. } => Some(Lint::UnevenPieceLength),
      Self::PieceLengthSmall { .. } => Some(Lint::SmallPieceLength),
      Self::PrivateTrackerless => Some(Lint::PrivateTrackerless),
      _ => None,
    }
  }

  pub(crate) fn internal(message: impl Into<String>) -> Error {
    Self::Internal {
      message: message.into(),
    }
  }
}

impl From<clap::Error> for Error {
  fn from(source: clap::Error) -> Self {
    Self::Clap { source }
  }
}

impl From<globset::Error> for Error {
  fn from(source: globset::Error) -> Self {
    Self::GlobParse { source }
  }
}

impl From<SystemTimeError> for Error {
  fn from(source: SystemTimeError) -> Self {
    Self::SystemTime { source }
  }
}

impl From<ignore::Error> for Error {
  fn from(ignore_error: ignore::Error) -> Self {
    Self::FileSearch {
      source: ignore_error,
    }
  }
}
