use crate::common::*;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub(crate) enum MagnetLinkParseError {
  #[snafu(display("Failed to parse hex string `{}`: {}", text, source))]
  HexParse {
    text: String,
    source: hex::FromHexError,
  },
  #[snafu(display("Hex-encoded infohash, `{}`, is not 40 characters long", text))]
  InfohashLength { text: String },
  #[snafu(display("Failed to parse peer address `{}`: {}", text, source))]
  PeerAddress {
    text: String,
    source: HostPortParseError,
  },
  #[snafu(display(
    "Invalid scheme: `{}`. Magnet links must use the `magnet:` scheme",
    scheme
  ))]
  Scheme { scheme: String },
  #[snafu(display("Magnet link must have a topic that begins with `urn:btih:`"))]
  TopicMissing,
  #[snafu(display("Failed to parse tracker address `{}`: {}", text, source))]
  TrackerAddress {
    text: String,
    source: url::ParseError,
  },
  #[snafu(display("Failed to parse URL: {}", source))]
  Url { source: url::ParseError },
}
