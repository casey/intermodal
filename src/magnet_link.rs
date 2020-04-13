use crate::common::*;

pub(crate) struct MagnetLink {
  infohash: Infohash,
  name: Option<String>,
  peers: Vec<HostPort>,
  trackers: Vec<Url>,
  indices: BTreeSet<u64>,
}

impl MagnetLink {
  pub(crate) fn from_metainfo(metainfo: &Metainfo) -> Result<MagnetLink> {
    let mut link = Self::with_infohash(metainfo.infohash()?);

    link.set_name(metainfo.info.name.clone());

    for tracker in metainfo.trackers() {
      link.add_tracker(tracker?);
    }

    Ok(link)
  }

  pub(crate) fn with_infohash(infohash: Infohash) -> MagnetLink {
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

  #[allow(dead_code)]
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
    let mut url = Url::parse("magnet:").unwrap();

    let mut query = format!("xt=urn:btih:{}", self.infohash);

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
}
