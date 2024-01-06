use crate::common::*;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct MagnetLink {
  pub(crate) infohash: Infohash,
  pub(crate) name: Option<String>,
  pub(crate) peers: Vec<HostPort>,
  pub(crate) trackers: Vec<Url>,
  pub(crate) indices: BTreeSet<u64>,
}

impl MagnetLink {
  /// See `Info::infohash_lossy` for details on when this function is lossy.
  pub(crate) fn from_metainfo_lossy(metainfo: &Metainfo) -> Result<MagnetLink> {
    let mut link = Self::with_infohash(metainfo.infohash_lossy()?);

    link.set_name(metainfo.info.name.clone());

    for tracker in metainfo.trackers() {
      link.add_tracker(tracker?);
    }

    Ok(link)
  }

  pub(crate) fn with_infohash(infohash: Infohash) -> Self {
    MagnetLink {
      infohash,
      name: None,
      peers: Vec::new(),
      trackers: Vec::new(),
      indices: BTreeSet::new(),
    }
  }

  #[allow(dead_code)]
  pub(crate) fn set_name(&mut self, name: impl Into<String>) {
    self.name = Some(name.into());
  }

  pub(crate) fn add_peer(&mut self, peer: HostPort) {
    self.peers.push(peer);
  }

  pub(crate) fn add_tracker(&mut self, tracker: Url) {
    self.trackers.push(tracker);
  }

  pub(crate) fn add_index(&mut self, index: u64) {
    self.indices.insert(index);
  }

  pub(crate) fn to_url(&self) -> Url {
    let mut url = Url::parse("magnet:").invariant_unwrap("`magnet:` is valid URL");

    let mut query = format!("xt=urn:btih:{}", self.infohash);

    if let Some(name) = &self.name {
      query.push_str("&dn=");
      query.push_str(name);
    }

    for tracker in &self.trackers {
      query.push_str("&tr=");
      query.push_str(tracker.as_str());
    }

    for peer in &self.peers {
      query.push_str("&x.pe=");
      query.push_str(&peer.to_string());
    }

    if !self.indices.is_empty() {
      query.push_str("&so=");
      for (i, selection_index) in self.indices.iter().enumerate() {
        if i > 0 {
          query.push(',');
        }
        query.push_str(&selection_index.to_string());
      }
    }

    url.set_query(Some(&query));

    url
  }

  fn parse(text: &str) -> Result<Self, MagnetLinkParseError> {
    let url = Url::parse(text).context(magnet_link_parse_error::Url)?;

    if url.scheme() != "magnet" {
      return Err(MagnetLinkParseError::Scheme {
        scheme: url.scheme().into(),
      });
    }

    let mut link = None;
    for (k, v) in url.query_pairs() {
      if k.as_ref() == "xt" {
        if let Some(infohash) = v.strip_prefix("urn:btih:") {
          if infohash.len() != 40 {
            return Err(MagnetLinkParseError::InfohashLength {
              text: infohash.into(),
            });
          }

          let buf = hex::decode(infohash).context(magnet_link_parse_error::HexParse {
            text: infohash.to_owned(),
          })?;

          link = Some(MagnetLink::with_infohash(
            Sha1Digest::from_bytes(
              buf
                .as_slice()
                .try_into()
                .invariant_unwrap("bounds are checked above"),
            )
            .into(),
          ));

          break;
        }
      }
    }

    let mut link = link.ok_or(MagnetLinkParseError::TopicMissing)?;

    for (k, v) in url.query_pairs() {
      match k.as_ref() {
        "tr" => link.add_tracker(Url::parse(&v).context(
          magnet_link_parse_error::TrackerAddress {
            text: v.to_string(),
          },
        )?),
        "dn" => link.set_name(v),
        "x.pe" => link.add_peer(HostPort::from_str(&v).context(
          magnet_link_parse_error::PeerAddress {
            text: v.to_string(),
          },
        )?),
        _ => {}
      }
    }

    Ok(link)
  }
}

impl FromStr for MagnetLink {
  type Err = Error;

  fn from_str(text: &str) -> Result<Self, Self::Err> {
    Self::parse(text).context(error::MagnetLinkParse { text })
  }
}

impl Display for MagnetLink {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{}", self.to_url())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use pretty_assertions::assert_eq;

  #[test]
  fn display() {
    let link = MagnetLink::with_infohash(Infohash::from_bencoded_info_dict("".as_bytes()));
    assert_eq!(
      link.to_string(),
      "magnet:?xt=urn:btih:da39a3ee5e6b4b0d3255bfef95601890afd80709"
    );
  }

  #[test]
  fn basic() {
    let link = MagnetLink::with_infohash(Infohash::from_bencoded_info_dict("".as_bytes()));
    assert_eq!(
      link.to_url().as_str(),
      "magnet:?xt=urn:btih:da39a3ee5e6b4b0d3255bfef95601890afd80709"
    );
  }

  #[test]
  fn with_name() {
    let mut link = MagnetLink::with_infohash(Infohash::from_bencoded_info_dict("".as_bytes()));
    link.set_name("foo");
    assert_eq!(
      link.to_url().as_str(),
      "magnet:?xt=urn:btih:da39a3ee5e6b4b0d3255bfef95601890afd80709&dn=foo"
    );
  }

  #[test]
  fn with_peer() {
    let mut link = MagnetLink::with_infohash(Infohash::from_bencoded_info_dict("".as_bytes()));
    link.add_peer("foo.com:1337".parse().unwrap());
    assert_eq!(
      link.to_url().as_str(),
      "magnet:?xt=urn:btih:da39a3ee5e6b4b0d3255bfef95601890afd80709&x.pe=foo.com:1337"
    );
  }

  #[test]
  fn with_tracker() {
    let mut link = MagnetLink::with_infohash(Infohash::from_bencoded_info_dict("".as_bytes()));
    link.add_tracker(Url::parse("http://foo.com/announce").unwrap());
    assert_eq!(
      link.to_url().as_str(),
      "magnet:?xt=urn:btih:da39a3ee5e6b4b0d3255bfef95601890afd80709&tr=http://foo.com/announce"
    );
  }

  #[test]
  fn with_indices() {
    let mut link = MagnetLink::with_infohash(Infohash::from_bencoded_info_dict("".as_bytes()));
    link.add_index(4);
    link.add_index(6);
    link.add_index(6);
    link.add_index(2);
    assert_eq!(
      link.to_url().as_str(),
      "magnet:?xt=urn:btih:da39a3ee5e6b4b0d3255bfef95601890afd80709&so=2,4,6"
    );
  }

  #[test]
  fn complex() {
    let mut link = MagnetLink::with_infohash(Infohash::from_bencoded_info_dict("".as_bytes()));
    link.set_name("foo");
    link.add_tracker(Url::parse("http://foo.com/announce").unwrap());
    link.add_tracker(Url::parse("http://bar.net/announce").unwrap());
    link.add_peer("foo.com:1337".parse().unwrap());
    link.add_peer("bar.net:666".parse().unwrap());
    assert_eq!(
      link.to_url().as_str(),
      concat!(
        "magnet:?xt=urn:btih:da39a3ee5e6b4b0d3255bfef95601890afd80709",
        "&dn=foo",
        "&tr=http://foo.com/announce",
        "&tr=http://bar.net/announce",
        "&x.pe=foo.com:1337",
        "&x.pe=bar.net:666",
      ),
    );
  }

  #[test]
  fn link_from_str_round_trip() {
    let mut link_to = MagnetLink::with_infohash(Infohash::from_bencoded_info_dict("".as_bytes()));

    link_to.set_name("foo");
    link_to.add_tracker(Url::parse("http://foo.com/announce").unwrap());
    link_to.add_tracker(Url::parse("http://bar.net/announce").unwrap());
    link_to.add_peer("foo.com:1337".parse().unwrap());
    link_to.add_peer("bar.net:666".parse().unwrap());

    let link_from = MagnetLink::from_str(link_to.to_url().as_ref()).unwrap();

    assert_eq!(link_to, link_from);
  }

  #[test]
  fn link_from_str_url_error() {
    let link = "%imdl.io";
    let e = MagnetLink::from_str(link).unwrap_err();

    assert_matches!(e, Error::MagnetLinkParse {
      text,
      source: MagnetLinkParseError::Url { .. },
    } if text == link);
  }

  #[test]
  fn link_from_str_scheme_error() {
    let link = "mailto:?alice@imdl.io";

    let e = MagnetLink::from_str(link).unwrap_err();
    assert_matches!(e, Error::MagnetLinkParse {
      text,
      source: MagnetLinkParseError::Scheme { scheme },
    } if text == link && scheme == "mailto");
  }

  #[test]
  fn link_from_str_infohash_length_error() {
    let infohash = "123456789abcedf";
    let link = format!("magnet:?xt=urn:btih:{infohash}");
    let e = MagnetLink::from_str(&link).unwrap_err();

    assert_matches!(e, Error::MagnetLinkParse {
      text,
      source: MagnetLinkParseError::InfohashLength { text: ih },
    } if text == link && infohash == ih);
  }

  #[test]
  fn link_from_str_infohash_bad_hex() {
    let infohash = "laaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    let link = format!("magnet:?xt=urn:btih:{infohash}");
    let e = MagnetLink::from_str(&link).unwrap_err();

    assert_matches!(e, Error::MagnetLinkParse {
      text,
      source: MagnetLinkParseError::HexParse {
        text: ih,
        ..
      }} if text == link && infohash == ih);
  }

  #[test]
  fn link_from_str_topic_missing() {
    let link = "magnet:?";
    let e = MagnetLink::from_str(link).unwrap_err();

    assert_matches!(e,
      Error::MagnetLinkParse {
        text,
        source: MagnetLinkParseError::TopicMissing,
      } if text == link);
  }

  #[test]
  fn link_from_str_tracker_address() {
    let infohash = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    let bad_addr = "%imdl.io/announce";
    let link = format!("magnet:?xt=urn:btih:{infohash}&tr={bad_addr}");
    let e = MagnetLink::from_str(&link).unwrap_err();

    assert_matches!(e,
      Error::MagnetLinkParse {
      text,
      source: MagnetLinkParseError::TrackerAddress {
        text: addr,
        ..
      },
    } if text == link && addr == bad_addr);
  }

  #[test]
  fn link_from_str_peer_address() {
    let infohash = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    let bad_addr = "%imdl.io:13337";
    let link = format!("magnet:?xt=urn:btih:{infohash}&x.pe={bad_addr}");
    let e = MagnetLink::from_str(&link).unwrap_err();

    assert_matches!(e,
      Error::MagnetLinkParse {
        text,
        source: MagnetLinkParseError::PeerAddress {
          text: addr,
          ..
        }
      } if text == link && addr == bad_addr
    );
  }
}
