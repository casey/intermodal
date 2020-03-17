use crate::common::*;

pub(crate) struct MagnetLink {
  info_hash: Sha1Digest,
  name: Option<String>,
  peers: Vec<HostPort>,
  trackers: Vec<Url>,
}

impl MagnetLink {
  pub(crate) fn new(info_hash: Sha1Digest) -> MagnetLink {
    MagnetLink {
      info_hash,
      name: None,
      peers: Vec::new(),
      trackers: Vec::new(),
    }
  }

  pub(crate) fn set_name(&mut self, name: impl Into<String>) {
    self.name = Some(name.into());
  }

  pub(crate) fn add_peer(&mut self, peer: HostPort) {
    self.peers.push(peer);
  }

  pub(crate) fn add_tracker(&mut self, tracker: Url) {
    self.trackers.push(tracker);
  }

  pub(crate) fn to_url(&self) -> Url {
    let mut url = Url::parse("magnet:").unwrap();

    let mut query = format!("xt=urn:btih:{}", self.info_hash);

    if let Some(name) = &self.name {
      query.push_str("&dn=");
      query.push_str(&name);
    }

    for tracker in &self.trackers {
      query.push_str("&tr=");
      query.push_str(tracker.as_str());
    }

    for peer in &self.peers {
      query.push_str("&x.pe=");
      query.push_str(&peer.to_string());
    }

    url.set_query(Some(&query));

    url
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  // TODO: make this import part of common
  use pretty_assertions::assert_eq;

  #[test]
  fn basic() {
    let link = MagnetLink::new(Sha1Digest::from_data(""));
    assert_eq!(
      link.to_url().as_str(),
      "magnet:?xt=urn:btih:da39a3ee5e6b4b0d3255bfef95601890afd80709"
    );
  }

  #[test]
  fn with_name() {
    let mut link = MagnetLink::new(Sha1Digest::from_data(""));
    link.set_name("foo");
    assert_eq!(
      link.to_url().as_str(),
      "magnet:?xt=urn:btih:da39a3ee5e6b4b0d3255bfef95601890afd80709&dn=foo"
    );
  }

  #[test]
  fn with_peer() {
    let mut link = MagnetLink::new(Sha1Digest::from_data(""));
    link.add_peer("foo.com:1337".parse().unwrap());
    assert_eq!(
      link.to_url().as_str(),
      "magnet:?xt=urn:btih:da39a3ee5e6b4b0d3255bfef95601890afd80709&x.pe=foo.com:1337"
    );
  }

  #[test]
  fn with_tracker() {
    let mut link = MagnetLink::new(Sha1Digest::from_data(""));
    link.add_tracker(Url::parse("http://foo.com/announce").unwrap());
    assert_eq!(
      link.to_url().as_str(),
      "magnet:?xt=urn:btih:da39a3ee5e6b4b0d3255bfef95601890afd80709&tr=http://foo.com/announce"
    );
  }

  #[test]
  fn complex() {
    let mut link = MagnetLink::new(Sha1Digest::from_data(""));
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
}
