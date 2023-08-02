use super::*;
use crate::common::*;

#[cfg(test)]
pub(crate) struct Daemon {
  pub(crate) sock: UdpSocket,
  pub(crate) records: HashMap<[u8; 20], HashSet<SocketAddr>>,
}

impl Daemon {
  pub fn spawn() -> (thread::JoinHandle<()>, SocketAddr) {
    Self::spawn_with_records(HashMap::new())
  }

  pub fn spawn_with_records(
    records: HashMap<[u8; 20], HashSet<SocketAddr>>,
  ) -> (thread::JoinHandle<()>, SocketAddr) {
    let sock = UdpSocket::bind((IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0)).unwrap();
    let addr = match sock.local_addr().unwrap() {
      SocketAddr::V4(a) => (Ipv4Addr::LOCALHOST, a.port()).into(),
      SocketAddr::V6(a) => (Ipv6Addr::LOCALHOST, a.port()).into(),
    };
    let mut d = Daemon { sock, records };
    let handle = thread::spawn(move || d.run());
    (handle, addr)
  }

  fn run(&mut self) {
    let mut rng = rand::thread_rng();
    let mut buf = [0u8; 8192];
    loop {
      if let Ok((n, peer)) = self.sock.recv_from(&mut buf) {
        if let Ok((req, _)) = connect::Request::deserialize(&buf[..n]) {
          let resp = connect::Response {
            action: Action::Connect.into(),
            transaction_id: req.transaction_id,
            connection_id: rng.gen(),
          }
          .serialize();
          self.sock.send_to(&resp, peer).unwrap();
          continue;
        }

        if let Ok((req, _)) = announce::Request::deserialize(&buf[..n]) {
          let mut resp: Vec<u8> = announce::Response {
            action: Action::Announce.into(),
            transaction_id: req.transaction_id,
            interval: 0x1337_1337,
            leechers: 0xcafe_babe,
            seeders: 0xdead_beef,
          }
          .serialize();
          resp.extend_from_slice(&self.peer_list(&req.infohash));
          self.sock.send_to(&resp, peer).unwrap();

          self.insert(req.infohash, peer);
        }
      }
    }
  }

  pub fn insert(&mut self, infohash: [u8; 20], addr: SocketAddr) {
    if let Some(set) = self.records.get_mut(&infohash) {
      set.insert(addr);
    } else {
      let mut set = HashSet::new();
      set.insert(addr);
      self.records.insert(infohash, set);
    }
  }

  fn peer_list(&self, infohash: &[u8; 20]) -> Vec<u8> {
    match self.records.get(infohash) {
      None => vec![],
      Some(set) => Self::compact_peer_list(set),
    }
  }

  fn compact_peer_list(set: &HashSet<SocketAddr>) -> Vec<u8> {
    let mut v = Vec::new();
    for p in set {
      match p.ip() {
        IpAddr::V4(ip) => v.extend_from_slice(&ip.octets()),
        IpAddr::V6(ip) => v.extend_from_slice(&ip.octets()),
      }
      v.extend_from_slice(&p.port().to_be_bytes());
    }
    v
  }
}

mod tests {
  use super::*;

  #[test]
  fn run() {
    let (_, addr) = Daemon::spawn();

    let mut c = Client::connect(addr).unwrap();
    let mut a = c.local_addr();
    let mut resp = c.announce_exchange(&[0u8; 20].into()).unwrap();

    for i in 0..4 {
      assert_eq!(resp.len(), i);
      c = Client::connect(addr).unwrap();
      resp = c.announce_exchange(&[0u8; 20].into()).unwrap();
      assert!(resp.contains(&a));
      a = c.local_addr();
    }
  }

  #[test]
  fn separate_infohashes() {
    let (_, addr) = Daemon::spawn();
    let c1 = Client::connect(addr).unwrap();
    let c2 = Client::connect(addr).unwrap();
    let a1 = c1.local_addr();
    let a2 = c2.local_addr();

    let infohash1 = Infohash::from(rand::thread_rng().gen::<[u8; 20]>());
    let infohash2 = Infohash::from(rand::thread_rng().gen::<[u8; 20]>());
    let resp1 = c1.announce_exchange(&infohash1).unwrap();
    let resp2 = c2.announce_exchange(&infohash2).unwrap();
    assert_eq!(resp2.len(), 0);
    assert_eq!(resp1.len(), 0);

    let resp1 = c1.announce_exchange(&infohash1).unwrap();
    let resp2 = c2.announce_exchange(&infohash2).unwrap();
    assert_eq!(resp1.len(), 1);
    assert_eq!(resp2.len(), 1);
    assert!(resp1.contains(&a1));
    assert!(resp2.contains(&a2));
  }

  #[test]
  fn reannounce() {
    let (_, addr) = Daemon::spawn();
    let infohash = Infohash::from(rand::thread_rng().gen::<[u8; 20]>());
    let c1 = Client::connect(addr).unwrap();
    let c2 = Client::connect(addr).unwrap();
    let a1 = c1.local_addr();
    let a2 = c2.local_addr();
    let resp1 = c1.announce_exchange(&infohash).unwrap();
    let resp2 = c2.announce_exchange(&infohash).unwrap();
    let resp3 = c1.announce_exchange(&infohash).unwrap();

    assert_eq!(resp1.len(), 0);
    assert_eq!(resp2.len(), 1);
    assert_eq!(resp3.len(), 2);
    assert!(resp3.contains(&a1));
    assert!(resp3.contains(&a2));
  }
}
