// The piece length picker attempts to pick a reasonable piece length
// for a torrent given the size of the torrent's contents.
//
// Constraints:
// - Decreasing piece length increases protocol overhead.
// - Decreasing piece length increases torrent metainfo size.
// - Increasing piece length increases the amount of data that must be
//   thrown away in case of corruption.
// - Increasing piece length increases the amount of data that must be
//   downloaded before it can be verified and uploaded to other peers.
// - Decreasing piece length increases the proportion of disk seeks to
//   disk reads. This can be an issue for spinning disks.
// - The BitTorrent v2 specification requires that piece sizes be
//   larger than 16 KiB.
//
// These constraints could probably be exactly defined and optimized
// using an integer programming solver, but instead we just copy what
// libtorrent does.

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
    let exponent = (content_size.count() as f64).log2().ceil() as u128;
    Bytes::from(1u128 << (exponent.max(20) / 2 + 4))
  }

  pub(crate) fn piece_count(content_size: Bytes, piece_length: Bytes) -> u128 {
    if content_size == Bytes::from(0u128) {
      0
    } else {
      (content_size / piece_length).max(1)
    }
  }

  pub(crate) fn metainfo_size(content_size: Bytes, piece_length: Bytes) -> Bytes {
    let digest_length: u128 = sha1::DIGEST_LENGTH.into_u64().into();
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
