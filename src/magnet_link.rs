use crate::common::*;

struct MagnetLink {
  info_hash: Sha1Digest,
  name: Option<String>,
  peers: Vec<Node>,
  trackers: Vec<Url>,
}

impl MagnetLink {
  fn new(info_hash: Sha1Digest) -> MagnetLink {
    MagnetLink {
      info_hash,
      name: None,
      peers: Vec::new(),
      trackers: Vec::new(),
    }
  }

  fn set_name(&mut self, name: impl Into<String>) {
    self.name = Some(name.into());
  }

  fn add_peer(&mut self, peer: Node) {
    self.peers.push(peer);
  }

  fn add_tracker(&mut self, tracker: Url) {
    self.trackers.push(tracker);
  }

  fn to_url(&self) -> Url {
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

// TODO:
// - rename node to HostPort, since it's used for peer addresses too

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn basic() {
    let link = MagnetLink::new(Sha1Digest::from_data(""));
    assert_eq!(
      link.to_url().as_str(),
      "magnet:?xt=urn:btih:da39a3ee5e6b4bd3255bfef95601890afd879"
    );
  }

  #[test]
  fn with_name() {
    let mut link = MagnetLink::new(Sha1Digest::from_data(""));
    link.set_name("foo");
    assert_eq!(
      link.to_url().as_str(),
      "magnet:?xt=urn:btih:da39a3ee5e6b4bd3255bfef95601890afd879&dn=foo"
    );
  }

  #[test]
  fn with_peer() {
    let mut link = MagnetLink::new(Sha1Digest::from_data(""));
    link.add_peer("foo.com:1337".parse().unwrap());
    assert_eq!(
      link.to_url().as_str(),
      "magnet:?xt=urn:btih:da39a3ee5e6b4bd3255bfef95601890afd879&x.pe=foo.com:1337"
    );
  }

  #[test]
  fn with_tracker() {
    let mut link = MagnetLink::new(Sha1Digest::from_data(""));
    link.add_tracker(Url::parse("http://foo.com/announce").unwrap());
    assert_eq!(
      link.to_url().as_str(),
      "magnet:?xt=urn:btih:da39a3ee5e6b4bd3255bfef95601890afd879&tr=http://foo.com/announce"
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
        "magnet:?xt=urn:btih:da39a3ee5e6b4bd3255bfef95601890afd879",
        "&dn=foo",
        "&tr=http://foo.com/announce",
        "&tr=http://bar.net/announce",
        "&x.pe=foo.com:1337",
        "&x.pe=bar.net:666",
      ),
    );
  }
}
