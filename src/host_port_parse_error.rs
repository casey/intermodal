use crate::common::*;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub(crate) enum HostPortParseError {
  #[snafu(display("Failed to parse host `{}`: {}", text, source))]
  Host {
    text: String,
    source: url::ParseError,
  },
  #[snafu(display("Failed to parse port `{}`: {}", text, source))]
  Port { text: String, source: ParseIntError },
  #[snafu(display("Port missing: `{}`", text))]
  PortMissing { text: String },
  #[snafu(display("Host missing: `{}`", text))]
  HostMissing { text: String },
  #[snafu(display("Host and port missing: `{}`", text))]
  HostPortMissing { text: String },
}
