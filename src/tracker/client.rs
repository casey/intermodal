use super::*;
use crate::common::*;

#[derive(Debug)]
pub(crate) struct Client {
  peer_id: [u8; 20],
  tracker_addr: SocketAddr,
  sock: UdpSocket,
  connection_id: Option<u64>,
}

impl Client {
  const RX_BUF_LEN: usize = 8192;
  const UDP_SOCKET_READ_TIMEOUT_S: u64 = 3;
  const UDP_SOCKET_READ_TIMEOUT_NS: u32 = 0;

  pub fn connect<A: ToSocketAddrs>(address: A) -> Result<Self> {
    let addrs = address
      .to_socket_addrs() // this may cause DNS look-ups!
      .context(error::TrackerSocketAddrs)?;

    for tracker_addr in addrs {
      let Ok(sock) = Self::new_udp_socket(tracker_addr) else {
        continue; // log these as warnings
      };
      let mut client = Client {
        peer_id: rand::thread_rng().gen(),
        tracker_addr,
        sock,
        connection_id: None,
      };
      if let Ok(()) = client.connect_exchange() {
        return Ok(client);
      }
    }
    Err(Error::TrackerNoHosts)
  }

  fn new_udp_socket(addr: SocketAddr) -> Result<UdpSocket> {
    let sock = match addr {
      SocketAddr::V4(_) => UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0)),
      SocketAddr::V6(_) => UdpSocket::bind((Ipv6Addr::UNSPECIFIED, 0)),
    }
    .context(error::UdpSocketBind)?;
    sock
      .connect(addr)
      .context(error::UdpSocketConnect { addr })?;
    sock
      .set_read_timeout(Some(Duration::new(
        Self::UDP_SOCKET_READ_TIMEOUT_S,
        Self::UDP_SOCKET_READ_TIMEOUT_NS,
      )))
      .context(error::UdpSocketReadTimeout)?;
    Ok(sock)
  }

  pub fn from_url(tracker_url: &Url) -> Result<Self> {
    if tracker_url.scheme() != "udp" {
      return Err(Error::TrackerUdpOnly {
        tracker_url: tracker_url.clone(),
      });
    }
    Self::connect(
      HostPort::try_from(tracker_url).context(error::TrackerHostPort {
        tracker_url: tracker_url.clone(),
      })?,
    )
  }

  fn connect_exchange(&mut self) -> Result<()> {
    let req = connect::Request::new();
    let mut buf = [0u8; connect::Response::LENGTH];
    let (resp, _) = self.exchange(&req, &mut buf)?;
    self.connection_id.replace(resp.connection_id);
    Ok(())
  }

  pub fn announce_exchange(&self, btinh: &Infohash) -> Result<Vec<SocketAddr>> {
    let Some(connection_id) = self.connection_id else {
      return Err(Error::TrackerNoConnectionId);
    };

    let local_addr = self
      .sock
      .local_addr()
      .context(error::UdpSocketLocalAddress)?;
    let req = announce::Request::new(connection_id, *btinh, self.peer_id, local_addr.port());
    let mut buf = [0u8; Self::RX_BUF_LEN];
    let (_, payload) = self.exchange(&req, &mut buf)?;

    Client::parse_compact_peer_list(payload, local_addr.is_ipv6())
  }

  fn exchange<'a, T: Request>(
    &self,
    req: &T,
    buf: &'a mut [u8],
  ) -> Result<(T::Response, &'a [u8])> {
    let msg = req.serialize();
    let mut len_read: usize = 0;

    for _ in 0..3 {
      self.sock.send(&msg).context(error::TrackerSend)?;
      if let Ok(len) = self.sock.recv(buf) {
        len_read = len;
        break;
      }
    }

    if len_read == 0 {
      return Err(Error::TrackerExchange {
        tracker_addr: self.tracker_addr,
      });
    }

    let (resp, payload) = T::Response::deserialize(&buf[..len_read])?;
    if resp.transaction_id() != req.transaction_id() || resp.action() != req.action() {
      return Err(Error::TrackerResponse);
    }

    Ok((resp, payload))
  }

  fn parse_compact_peer_list(buf: &[u8], is_ipv6: bool) -> Result<Vec<SocketAddr>> {
    let mut peer_list = Vec::<SocketAddr>::new();
    let stride = if is_ipv6 { 18 } else { 6 };

    let chunks = buf.chunks_exact(stride);
    if !chunks.remainder().is_empty() {
      return Err(Error::TrackerCompactPeerList);
    }

    for hostpost in chunks {
      let (ip, port) = hostpost.split_at(stride - 2);
      let ip = if is_ipv6 {
        let octets: [u8; 16] = ip[0..16]
          .try_into()
          .invariant_unwrap("iterator guarantees bounds are OK");
        IpAddr::from(std::net::Ipv6Addr::from(octets))
      } else {
        IpAddr::from(std::net::Ipv4Addr::new(ip[0], ip[1], ip[2], ip[3]))
      };

      let port = u16::from_be_bytes(
        port
          .try_into()
          .invariant_unwrap("iterator guarantees bounds are OK"),
      );

      peer_list.push((ip, port).into());
    }

    Ok(peer_list)
  }

  #[cfg(test)]
  pub fn local_addr(&self) -> SocketAddr {
    (Ipv4Addr::LOCALHOST, self.sock.local_addr().unwrap().port()).into()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct TestServer {
    sock: UdpSocket,
    peer_list: Vec<u8>,
  }

  impl TestServer {
    fn new_ipv4() -> (Self, SocketAddr, Vec<u8>) {
      TestServer::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED))
    }

    fn new_ipv6() -> (Self, SocketAddr, Vec<u8>) {
      TestServer::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED))
    }

    fn new(addr: IpAddr) -> (Self, SocketAddr, Vec<u8>) {
      let sock = UdpSocket::bind((addr, 0)).unwrap();
      sock.set_read_timeout(None).unwrap();

      let server_addr = sock.local_addr().unwrap();
      let stride = if server_addr.is_ipv6() { 18 } else { 6 };
      let peer_list: Vec<u8> = (0..10 * stride)
        .map(|_| rand::thread_rng().gen::<u8>())
        .collect::<Vec<_>>();

      let local_addr = if server_addr.is_ipv6() {
        (Ipv6Addr::LOCALHOST, server_addr.port()).into()
      } else {
        (Ipv4Addr::LOCALHOST, server_addr.port()).into()
      };

      (
        TestServer {
          sock,
          peer_list: peer_list.clone(),
        },
        local_addr,
        peer_list,
      )
    }

    fn connect_exchange(&self) {
      let mut buf = [0u8; 8192];
      let mut rng = rand::thread_rng();

      let (n, peer) = self.sock.recv_from(&mut buf).unwrap();
      let (req, _) = connect::Request::deserialize(buf[..n].try_into().unwrap()).unwrap();
      let req = connect::Response {
        action: Action::Connect.into(),
        transaction_id: req.transaction_id,
        connection_id: rng.gen(),
      }
      .serialize();
      self.sock.send_to(&req, peer).unwrap();
    }

    fn announce_exchange(&self) {
      let mut buf = [0u8; 8192];

      let (n, peer) = self.sock.recv_from(&mut buf).unwrap();
      let (req, _) = announce::Request::deserialize(&buf[..n]).unwrap();
      let mut req: Vec<u8> = announce::Response {
        action: Action::Announce.into(),
        transaction_id: req.transaction_id,
        interval: 0x1337_1337,
        leechers: 0xcafe_babe,
        seeders: 0xdead_beef,
      }
      .serialize();
      req.extend_from_slice(&self.peer_list);
      self.sock.send_to(&req, peer).unwrap();
    }
  }

  #[test]
  fn client_from_url_no_port() {
    let tracker_url = Url::parse("udp://intermodal.io/announce").unwrap();
    assert_matches!(
      Client::from_url(&tracker_url),
      Err(Error::TrackerHostPort { .. })
    );
  }

  #[test]
  fn client_from_url_no_host() {
    let tracker_url = Url::parse("udp://magnet:?announce=no_host").unwrap();
    assert_matches!(
      Client::from_url(&tracker_url),
      Err(Error::TrackerHostPort { .. })
    );
  }

  #[test]
  fn client_from_url_not_udp() {
    let tracker_url = Url::parse("https://intermodal.io:100/announce").unwrap();
    assert_matches!(
      Client::from_url(&tracker_url),
      Err(Error::TrackerUdpOnly { .. })
    );
  }

  #[test]
  fn client_connect_v4() {
    let (server, addr, _) = TestServer::new_ipv4();
    thread::spawn(move || {
      server.connect_exchange();
    });
    Client::connect(addr).unwrap();
  }

  #[test]
  fn client_connect_v6() {
    let (server, addr, _) = TestServer::new_ipv6();
    thread::spawn(move || {
      server.connect_exchange();
    });
    Client::connect(addr).unwrap();
  }

  #[test]
  fn client_connect_timeout_ipv4() {
    let (_, addr, _) = TestServer::new_ipv4();
    assert_matches!(Client::connect(addr), Err(Error::TrackerNoHosts { .. }));
  }

  #[test]
  fn client_connect_timeout_ipv6() {
    let (_, addr, _) = TestServer::new_ipv6();
    assert_matches!(Client::connect(addr), Err(Error::TrackerNoHosts { .. }));
  }

  #[test]
  fn client_announce_without_connection_id() {}

  #[test]
  fn client_announce_timeout_ipv4() {
    let (server, addr, _) = TestServer::new_ipv4();
    thread::spawn(move || {
      server.connect_exchange();
    });

    let c = Client::connect(addr).unwrap();
    let addrs = c.announce_exchange(&Sha1Digest::from_bytes([0u8; 20]).into());
    assert_matches!(addrs, Err(Error::TrackerExchange { .. }));
  }

  #[test]
  fn client_announce_timeout_ipv6() {
    let (server, addr, _) = TestServer::new_ipv4();
    thread::spawn(move || {
      server.connect_exchange();
    });

    let c = Client::connect(addr).unwrap();
    let addrs = c.announce_exchange(&Sha1Digest::from_bytes([0u8; 20]).into());
    assert_matches!(addrs, Err(Error::TrackerExchange { .. }));
  }

  #[test]
  fn client_announce_ipv4() {
    let (server, addr, expected_targets) = TestServer::new_ipv4();
    thread::spawn(move || {
      server.connect_exchange();
      server.announce_exchange();
    });

    let c = Client::connect(addr).unwrap();
    let addrs = c
      .announce_exchange(&Sha1Digest::from_bytes([0u8; 20]).into())
      .unwrap();
    assert_eq!(
      addrs,
      Client::parse_compact_peer_list(&expected_targets, addr.is_ipv6()).unwrap()
    );
  }

  #[test]
  fn client_announce_ipv6() {
    let (server, addr, expected_targets) = TestServer::new_ipv6();
    thread::spawn(move || {
      server.connect_exchange();
      server.announce_exchange();
    });

    let c = Client::connect(addr).unwrap();
    let addrs = c
      .announce_exchange(&Sha1Digest::from_bytes([0u8; 20]).into())
      .unwrap();
    assert_eq!(
      addrs,
      Client::parse_compact_peer_list(&expected_targets, addr.is_ipv6()).unwrap()
    );
  }
}
