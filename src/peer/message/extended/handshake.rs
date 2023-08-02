use crate::common::*;

use extended::ut_metadata;
use peer::message::extended;

// From BEP10:
//
// Handshake is a dictionary of supported extension messages which maps names of extensions to an
// extended message ID for each extension message. The only requirement on these IDs is that no
// extension message share the same one. Setting an extension number to zero means that the
// extension is not supported/disabled. The client should ignore any extension names it doesn't
// recognize.
//
// The extension message IDs are the IDs used to send the extension messages to the peer sending
// this handshake. i.e. The IDs are local to this particular peer.
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Handshake {
  #[serde(rename = "m")]
  pub(crate) message_ids: HashMap<String, u8>,
  // Sent with the handshake if ut_metadata is supported.
  #[serde(
    default,
    skip_serializing_if = "Option::is_none",
    with = "unwrap_or_skip"
  )]
  pub(crate) metadata_size: Option<usize>,
  // Local TCP listen port. Allows each side to learn about the TCP port number
  // of the other side.  Note that there is no need for the receiving side of
  // the connection to send this extension message, since its port number is
  // already known.
  #[serde(
    default,
    rename = "p",
    skip_serializing_if = "Option::is_none",
    with = "unwrap_or_skip"
  )]
  pub(crate) port: Option<u16>,
  // Client name and version (as a utf-8 string). This is a much more reliable
  // way of identifying the client than relying on the peer id encoding.
  #[serde(
    default,
    rename = "v",
    skip_serializing_if = "Option::is_none",
    with = "unwrap_or_skip"
  )]
  pub(crate) version: Option<String>,
  // A string containing the compact representation of the ip address this peer
  // sees you as. i.e.  this is the receiver's external ip address (no port is
  // included). This may be either an IPv4 (4 bytes) or an IPv6 (16 bytes)
  // address.
  #[serde(default, skip_serializing_if = "Vec::is_empty", with = "serde_bytes")]
  pub(crate) yourip: Vec<u8>,
  // If this peer has an IPv6 interface, this is the compact representation of
  // that address (16 bytes). The client may prefer to connect back via the IPv6
  // address.
  #[serde(default, skip_serializing_if = "Vec::is_empty", with = "serde_bytes")]
  pub(crate) ipv6: Vec<u8>,
  // If this peer has an IPv4 interface, this is the compact representation of
  // that address (4 bytes). The client may prefer to connect back via this
  // interface.
  #[serde(default, skip_serializing_if = "Vec::is_empty", with = "serde_bytes")]
  pub(crate) ipv4: Vec<u8>,
  // An integer, the number of outstanding request messages this client supports
  // without dropping any. The default in in libtorrent is 250.
  #[serde(
    default,
    rename = "reqq",
    skip_serializing_if = "Option::is_none",
    with = "unwrap_or_skip"
  )]
  pub(crate) request_queue_size: Option<u64>,
}

impl Handshake {
  pub(crate) fn new() -> Self {
    Handshake {
      message_ids: HashMap::new(),
      metadata_size: None,
      port: None,
      ipv4: vec![],
      ipv6: vec![],
      request_queue_size: None,
      yourip: vec![],
      version: Some(format!("intermodal {}", consts::VERSION)),
    }
  }

  pub(crate) fn with_message(&mut self, name: String, id: u8) {
    self.message_ids.insert(name, id);
  }

  pub(crate) fn with_metadata_size(&mut self, size: usize) {
    self.metadata_size = Some(size);
  }
}

impl Default for Handshake {
  fn default() -> Self {
    let mut handshake = Handshake::new();
    handshake.with_message(String::from(ut_metadata::UtMetadata::NAME), 1);
    handshake
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn handshake_from_bencoded_handshake() {
    let payload = b"d1:ei0e1:md11:ut_metadatai255ee13:metadata_sizei1337e1:pi12345e4:reqqi2048e1:v18:intermodal v0.1.12e";
    let handshake: Handshake = bendy::serde::de::from_bytes(payload).unwrap();
    assert_eq!(
      handshake
        .message_ids
        .get(&String::from(extended::Id::UtMetadata)),
      Some(255).as_ref()
    );
    assert_eq!(handshake.metadata_size, Some(1337));
    assert_eq!(handshake.port, Some(12345));
    assert_eq!(handshake.version, Some("intermodal v0.1.12".into()));
  }

  #[test]
  fn handshake_with_unsupported_extensions() {
    let payload = b"d12:complete_agoi6e1:md11:lt_donthavei7e10:share_modei8e11:upload_onlyi3e12:ut_holepunchi4e11:ut_metadatai2e6:ut_pexi1eee";
    let handshake: Handshake = bendy::serde::de::from_bytes(payload).unwrap();
    assert_eq!(
      handshake
        .message_ids
        .get(&String::from(extended::Id::UtMetadata)),
      Some(2).as_ref()
    );
    assert_eq!(handshake.metadata_size, None);
  }

  #[test]
  fn handshake_with_yourip() {
    let payload = b"d1:ei0e1:md11:ut_metadatai255ee13:metadata_sizei1337e1:pi12345e4:reqqi2048e1:v18:intermodal v0.1.126:yourip4:z\xc7%\xcfee";
    let handshake: Handshake = bendy::serde::de::from_bytes(payload).unwrap();
    assert_eq!(
      handshake
        .message_ids
        .get(&String::from(extended::Id::UtMetadata)),
      Some(255).as_ref()
    );
    assert_eq!(handshake.yourip, vec![0x7a, 0xc7, 0x25, 0xcf]);
  }

  #[test]
  fn handshake_ser_with_yourip() {
    let handshake = Handshake {
      ipv4: vec![0x13, 0x37, 0x13, 0x37],
      ..Handshake::default()
    };
    bendy::serde::ser::to_bytes(&handshake).unwrap();
  }

  #[test]
  fn extension_handshake_default_ok() {
    let handshake = Handshake::default();
    bendy::serde::ser::to_bytes(&handshake).unwrap();
  }
}
