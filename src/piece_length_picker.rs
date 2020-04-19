//! See [the book](https://imdl.io/book/bittorrent/piece-length-selection.html)
//! for more information on Intermodal's automatic piece length selection
//! algorithm.

use crate::common::*;

pub(crate) struct PieceLengthPicker;

impl PieceLengthPicker {
  pub(crate) fn from_content_size(content_size: Bytes) -> Bytes {
    #![allow(
      clippy::as_conversions,
      clippy::cast_sign_loss,
      clippy::cast_precision_loss,
      clippy::cast_possible_truncation
    )]
    let exponent = (content_size.count().max(1) as f64).log2().ceil() as u64;
    Bytes::from(1u64 << (exponent / 2 + 4))
      .max(Bytes::kib() * 16)
      .min(Bytes::mib() * 16)
  }

  pub(crate) fn piece_count(content_size: Bytes, piece_length: Bytes) -> u64 {
    if content_size == Bytes::from(0u64) {
      0
    } else {
      (content_size / piece_length).max(1)
    }
  }

  pub(crate) fn metainfo_size(content_size: Bytes, piece_length: Bytes) -> Bytes {
    let digest_length: u64 = sha1::DIGEST_LENGTH.into_u64();
    Bytes::from(Self::piece_count(content_size, piece_length) * digest_length)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn limits() {
    assert_eq!(
      PieceLengthPicker::from_content_size(Bytes::mib() * 2),
      Bytes::kib() * 16
    );
    assert_eq!(
      PieceLengthPicker::from_content_size(Bytes::mib() * 4),
      Bytes::kib() * 32
    );
    assert_eq!(
      PieceLengthPicker::from_content_size(Bytes::mib() * 8),
      Bytes::kib() * 32
    );
    assert_eq!(
      PieceLengthPicker::from_content_size(Bytes::mib() * 16),
      Bytes::kib() * 64
    );
  }
}
