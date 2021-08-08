use super::*;
use crate::common::*;

pub(crate) struct Client {
  connection_id: Option<u64>,
  peer_id: [u8; 20],
  is_ipv6: bool,
  sock: UdpSocket,
  host_port: HostPort,
}

impl<'a> Client {
  const UDP_TRACKER_MAGIC: u64 = 0x0000_0417_2710_1980;

  pub fn connect(host_port: &HostPort) -> Result<Self> {
    let mut rng = rand::thread_rng();

    let is_ipv6;
    let sock;
    let mut addrs = host_port
      .to_socket_addrs()
      .context(error::TrackerDnsResolution {
        host_port: host_port.clone(),
      })?;
    if let Some(addr) = addrs.next() {
      sock = match addr {
        SocketAddr::V4(_) => {
          is_ipv6 = false;
          UdpSocket::bind("0.0.0.0:0").context(error::Network)?
        }
        SocketAddr::V6(_) => {
          is_ipv6 = true;
          UdpSocket::bind("[::1]:0").context(error::Network)?
        }
      };
      sock.connect(addr).context(error::Network)?;
      // Set the read timeout to 500ms; anything else is way too slow.
      sock
        .set_read_timeout(Some(Duration::new(0, 500_000_000)))
        .context(error::Network)?;
    } else {
      return Err(Error::TrackerNoHosts {
        host_port: host_port.clone(),
      });
    }

    let mut client = Client {
      peer_id: rng.gen(),
      connection_id: None,
      sock,
      is_ipv6,
      host_port: host_port.clone(),
    };

    let req = connect::Request {
      protocol_id: Self::UDP_TRACKER_MAGIC,
      action: 0,
      transaction_id: rng.gen(),
    };
    let mut buf = [0u8; announce::Response::LENGTH];
    let (resp, _) = client.exchange(&req, &mut buf)?;
    client.connection_id.replace(resp.connection_id);

    Ok(client)
  }

  pub fn announce(&self, btinh: Infohash) -> Result<Vec<SocketAddr>> {
    let mut rng = rand::thread_rng();
    let connection_id;
    if let Some(id) = self.connection_id {
      connection_id = id;
    } else {
      return Err(Error::TrackerResponse);
    }
    let req = announce::Request {
      connection_id,
      action: 0x0001,
      transaction_id: rng.gen(),
      infohash: btinh.into(),
      peer_id: self.peer_id,
      downloaded: 0x0000,
      left: u64::MAX,
      uploaded: 0x0000,
      event: 0x0000,
      ip_address: 0x0000,
      num_want: u32::MAX,
      port: self.sock.local_addr().context(error::Network)?.port(),
    };
    let mut buf = [0u8; 1024];
    let (_, payload) = self.exchange(&req, &mut buf)?;

    Client::parse_compact_peer_list(self.is_ipv6, payload)
  }

  fn exchange<T: Request>(&self, req: &T, rxbuf: &'a mut [u8]) -> Result<(T::Response, &'a [u8])> {
    let msg = req.serialize();
    let mut len_read: usize = 0;

    for _ in 0..3 {
      self.sock.send(&msg).context(error::Network)?;
      if let Ok(len) = self.sock.recv(rxbuf) {
        len_read = len;
        break;
      }
    }

    if len_read == 0 {
      return Err(Error::TrackerTimeout {
        host_port: self.host_port.clone(),
      });
    }

    let (resp, payload) = T::Response::deserialize(&rxbuf[..len_read])?;
    if resp.transaction_id() != req.transaction_id() || resp.action() != req.action() {
      return Err(Error::TrackerResponse);
    }

    Ok((resp, payload))
  }

  fn parse_compact_peer_list(is_ipv6: bool, buf: &[u8]) -> Result<Vec<SocketAddr>> {
    let mut subswarm = Vec::<SocketAddr>::new();
    let stride = if is_ipv6 { 18 } else { 6 };

    let chunks = buf.chunks_exact(stride);
    if !chunks.remainder().is_empty() {
      return Err(Error::TrackerResponse);
    }

    for hostpost in chunks {
      let (ip, port) = hostpost.split_at(stride - 2);
      let ip = if is_ipv6 {
        let buf: [u8; 16] = ip[0..16]
          .try_into()
          .invariant_unwrap("iterator guarantees bounds are OK");
        IpAddr::from(std::net::Ipv6Addr::from(buf))
      } else {
        IpAddr::from(std::net::Ipv4Addr::new(ip[0], ip[1], ip[2], ip[3]))
      };

      let port = u16::from_be_bytes(
        port
          .try_into()
          .invariant_unwrap("iterator guarantees bounds are OK"),
      );

      subswarm.push((ip, port).into());
    }

    Ok(subswarm)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use action::Action;
  use std::{thread, time::Duration};

  #[cfg(test)]
  pub(crate) fn dummy_metainfo() -> Metainfo {
    Metainfo {
      announce: None,
      announce_list: None,
      nodes: None,
      comment: None,
      created_by: None,
      creation_date: None,
      encoding: None,
      info: Info {
        private: None,
        piece_length: Bytes(16 * 1024),
        source: None,
        name: "testing".into(),
        pieces: PieceList::from_pieces(&["test", "data"]),
        mode: Mode::Single {
          length: Bytes(2 * 16 * 1024),
          md5sum: None,
        },
        update_url: None,
      },
    }
  }

  fn tracker_simulate_announce_response(server: UdpSocket, targets: Vec<u8>) {
    tracker_simulate_announce_response_helper(server, targets, false)
  }

  fn tracker_simulate_announce_response_slowly(server: UdpSocket, targets: Vec<u8>) {
    tracker_simulate_announce_response_helper(server, targets, true)
  }

  fn tracker_simulate_announce_response_helper(server: UdpSocket, targets: Vec<u8>, slowly: bool) {
    let mut rng = rand::thread_rng();
    let mut buf = [0u8; 8192];

    // connect exchange
    let (n, peer) = server.recv_from(&mut buf).unwrap();
    let (req, _) = connect::Request::deserialize(buf[..n].try_into().unwrap()).unwrap();
    let req = connect::Response {
      action: Action::Connect.into(),
      transaction_id: req.transaction_id,
      connection_id: rng.gen(),
    }
    .serialize();
    if slowly {
      thread::sleep(Duration::new(60, 0));
    }
    server.send_to(&req, peer).unwrap();

    // announce exchange
    let (_n, peer) = server.recv_from(&mut buf).unwrap();
    let (req, _) = announce::Request::deserialize(&buf).unwrap();
    let mut req: Vec<u8> = announce::Response {
      action: Action::Announce.into(),
      transaction_id: req.transaction_id,
      interval: 0x1337_1337,
      leechers: 0xcafe_babe,
      seeders: 0xdead_beef,
    }
    .serialize();
    req.extend_from_slice(&targets);
    if slowly {
      thread::sleep(Duration::new(60, 0));
    }
    server.send_to(&req, peer).unwrap();
  }

  fn client_end_to_end_announce_helper(addr: &str, slow: bool) {
    let server = UdpSocket::bind(addr).unwrap();
    server.set_read_timeout(Some(Duration::new(15, 0))).unwrap();

    let mut metainfo = dummy_metainfo();
    let server_local_addr = server.local_addr().unwrap();
    metainfo.announce = Some(format!("udp://{}", server_local_addr));
    let mut env = test_env! {
      args: [
        "torrent",
        "announce",
        "--input",
        "test.torrent",
      ],
      tree: {
      }
    };
    env.write("test.torrent", metainfo.serialize().unwrap());

    let targets = if server_local_addr.is_ipv6() {
      vec![
        0x13, 0x37, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0xca, 0xfe,
      ]
    } else {
      vec![0x13, 0x37, 0xca, 0xfe, 0xba, 0xbe]
    };

    let target_sockaddrs = Client::parse_compact_peer_list(server_local_addr.is_ipv6(), &targets)
      .unwrap()
      .into_iter()
      .map(|i| i.to_string())
      .collect::<Vec<String>>()
      .join("\n");

    // simulate a tracker response
    if slow {
      thread::spawn(move || {
        tracker_simulate_announce_response_slowly(server, targets);
      });
    } else {
      thread::spawn(move || {
        tracker_simulate_announce_response(server, targets);
      });
    }
    env.run().unwrap();

    if slow {
      assert_eq!(
        env.err(),
        format!(
          "Couldn't connect to tracker: Connection to UDP tracker `{}` timed out.\n",
          server_local_addr
        )
      );
    } else {
      assert_eq!(env.out(), format!("{}\n", target_sockaddrs));
    }
  }

  #[test]
  fn client_end_to_end_announce_hostname() {
    client_end_to_end_announce_helper("localhost:0", false);
  }

  #[test]
  fn client_end_to_end_announce_ipv4() {
    client_end_to_end_announce_helper("127.0.0.1:0", false);
  }

  #[test]
  fn client_end_to_end_announce_ipv6() {
    client_end_to_end_announce_helper("[::1]:0", false)
  }

  #[test]
  #[ignore]
  fn client_end_to_end_announce_ipv6_timeout() {
    client_end_to_end_announce_helper("[::1]:0", true)
  }

  #[test]
  fn client_end_to_end_announce_timeout() {
    client_end_to_end_announce_helper("127.0.0.1:0", true)
  }
}
