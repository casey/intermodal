use crate::common::*;

use message::extended;
use message::Message;
use peer::connection::Connection;
use peer::message;

#[derive(Debug)]
pub(crate) struct Client {
  infohash: Infohash,
  conn: Connection,
  state: State,
  info: Option<Info>,
  extension_handshake: Option<extended::Handshake>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum State {
  Idle,
  WantInfo(Vec<u8>),
}

impl Client {
  pub(crate) fn connect(addr: &SocketAddr, infohash: Infohash) -> Result<Self> {
    let conn = Connection::new(addr, infohash)?;

    if !conn.supports_extension_protocol() {
      return Err(Error::PeerUtMetadataNotSupported);
    }

    Ok(Client {
      infohash,
      conn,
      state: State::Idle,
      info: None,
      extension_handshake: None,
    })
  }

  fn send_extension_handshake(&mut self) -> Result<()> {
    let mut handshake = extended::Handshake::default();
    if let Some(info) = &self.info {
      let info_dict = bendy::serde::ser::to_bytes(&info).context(error::InfoSerialize)?;
      handshake.with_metadata_size(info_dict.len());
    }
    self.conn.send(&Message::new_extended(
      extended::Id::Handshake.into(),
      handshake,
    )?)
  }

  pub(crate) fn fetch_info_dict(mut self) -> Result<Info> {
    self.state = State::WantInfo(Vec::new());

    self.send_extension_handshake()?;

    loop {
      if let Some(info) = self.info {
        return Ok(info);
      }

      let msg = self.conn.recv()?;
      if msg.flavour != message::Flavour::Extended {
        continue;
      }

      self.handle_msg(&msg)?;
    }
  }

  fn handle_msg(&mut self, msg: &Message) -> Result<()> {
    match msg.flavour {
      message::Flavour::Extended => self.handle_extended(msg),
      _ => Ok(()),
    }
  }

  fn handle_extended(&mut self, msg: &Message) -> Result<()> {
    let (id, payload) = msg.parse_extended_payload()?;
    match id {
      extended::Id::Handshake => self.handle_extension_handshake(payload),
      extended::Id::UtMetadata => self.handle_ut_metadata(payload),
      extended::Id::NotImplemented(_) => Ok(()),
    }
  }

  fn handle_extension_handshake(&mut self, payload: &[u8]) -> Result<()> {
    let handshake: extended::Handshake = Message::from_bencode(payload)?;

    // Drop the peer if we want info and the peer can't give it to us.
    if let State::WantInfo(_) = self.state {
      if handshake.metadata_size.is_none() {
        return Err(Error::PeerUtMetadataMetadataSizeNotKnown);
      } else if !handshake
        .message_ids
        .contains_key(extended::UtMetadata::NAME)
      {
        return Err(Error::PeerUtMetadataNotSupported);
      }
    };

    self.extension_handshake.replace(handshake);

    if let State::WantInfo(_) = self.state {
      self.send_ut_metadata_request(0)?;
    }

    Ok(())
  }

  fn handle_ut_metadata(&mut self, payload: &[u8]) -> Result<()> {
    let metadata_size = self.ut_metadata_size()?;

    let msg: extended::UtMetadata = Message::from_bencode(payload)?;
    match msg.msg_type.into() {
      extended::ut_metadata::MsgType::Data => (),
      extended::ut_metadata::MsgType::Request | extended::ut_metadata::MsgType::Reject => {
        return Ok(())
      }
    };

    if let State::WantInfo(info_buf) = &mut self.state {
      let piece = info_buf.len() / extended::UtMetadata::PIECE_LENGTH;
      if msg.piece != piece {
        return Err(Error::PeerUtMetadataWrongPiece);
      }

      // The ut_metadata::MsgType::Data payload splits into two parts,
      // 1. a bencoded UtMetadata message,
      // 2. the binary info_dict peice data.
      // Their boundary is not delimited. Bencode the message to find the piece offset.
      let piece_offset = bendy::serde::ser::to_bytes(&msg)
        .context(error::PeerMessageBencode)?
        .len();
      if payload[piece_offset..].len() > extended::UtMetadata::PIECE_LENGTH {
        return Err(Error::PeerUtMetadataPieceLength);
      }
      info_buf.extend_from_slice(&payload[piece_offset..]);

      return match info_buf.len().cmp(&metadata_size) {
        Ordering::Equal => {
          let info = Self::verify_info_dict(info_buf, self.infohash)?;
          self.info = Some(info);
          self.state = State::Idle;
          Ok(())
        }
        Ordering::Less => self.send_ut_metadata_request(piece + 1),
        Ordering::Greater => Err(Error::PeerUtMetadataInfoLength),
      };
    }

    Ok(())
  }

  pub(crate) fn send_ut_metadata_request(&mut self, piece: usize) -> Result<()> {
    let id = self.ut_metadata_msg_id()?;
    let msg = Message::new_extended(id, extended::UtMetadata::request(piece))?;

    self.conn.send(&msg)
  }

  fn verify_info_dict(buf: &[u8], target: Infohash) -> Result<Info> {
    let info =
      bendy::serde::de::from_bytes::<Info>(buf).context(error::PeerUtMetadataInfoDeserialize)?;

    let infohash = Infohash::from_bencoded_info_dict(
      &bendy::serde::ser::to_bytes(&info).context(error::InfoSerialize)?,
    );

    if infohash == target {
      Ok(info)
    } else {
      Err(Error::PeerUtMetadataWrongInfohash)
    }
  }

  fn ut_metadata_size(&self) -> Result<usize> {
    match &self.extension_handshake {
      Some(handshake) => match handshake.metadata_size {
        Some(size) => Ok(size),
        None => Err(Error::PeerUtMetadataMetadataSizeNotKnown),
      },
      None => Err(Error::PeerNoExtendedHandshake),
    }
  }

  fn ut_metadata_msg_id(&self) -> Result<u8> {
    match &self.extension_handshake {
      Some(handshake) => match handshake.message_ids.get(extended::UtMetadata::NAME) {
        Some(id) => Ok(*id),
        None => Err(Error::PeerUtMetadataNotSupported),
      },
      None => Err(Error::PeerNoExtendedHandshake),
    }
  }

  #[cfg(test)]
  pub(crate) fn listen(listener: &TcpListener, infohash: Infohash) -> Result<Self> {
    let (conn, _) = listener.accept().context(error::Network)?;
    conn
      .set_read_timeout(Some(Duration::new(3, 0)))
      .context(error::Network)?;

    Ok(Client {
      infohash,
      conn: Connection::from(conn, infohash)?,
      state: State::Idle,
      extension_handshake: None,
      info: None,
    })
  }

  #[cfg(test)]
  pub(crate) fn send_ut_metadata_data(
    &mut self,
    piece: usize,
    total_size: usize,
    data: &[u8],
  ) -> Result<()> {
    let message_id = self.ut_metadata_msg_id()?;

    let msg = Message::new_extended_with_trailer(
      message_id,
      extended::UtMetadata::data(piece, total_size),
      data,
    )?;

    self.conn.send(&msg)
  }

  #[cfg(test)]
  pub(crate) fn spawn_info_dict_seeder(info: &Info) -> (thread::JoinHandle<()>, SocketAddr) {
    let info_dict = bendy::serde::ser::to_bytes(info).unwrap();
    let infohash = Infohash::from_bencoded_info_dict(&info_dict);
    let listener = TcpListener::bind((Ipv4Addr::UNSPECIFIED, 0)).unwrap();
    let addr = (Ipv4Addr::LOCALHOST, listener.local_addr().unwrap().port()).into();
    let seeder = thread::spawn(move || {
      let mut seeder = Client::listen(&listener, infohash).unwrap();
      let handshake = extended::Handshake {
        metadata_size: Some(info_dict.len()),
        ..extended::Handshake::default()
      };
      let msg = Message::new_extended(extended::Id::Handshake.into(), handshake).unwrap();
      seeder.conn.send(&msg).unwrap();

      // The first message from the fetcher is an extension handshake.
      let msg = seeder.conn.recv().unwrap();
      assert_eq!(msg.flavour, message::Flavour::Extended);
      let (id, _) = msg.parse_extended_payload().unwrap();
      assert_eq!(id, extended::Id::Handshake);
      seeder.handle_msg(&msg).unwrap();

      let mut pieces = info_dict.len() / extended::UtMetadata::PIECE_LENGTH;
      if info_dict.len() % extended::UtMetadata::PIECE_LENGTH > 0 {
        pieces += 1;
      }

      // Respond to any serviceable ut_metadata request. Ignore errors.
      loop {
        let Ok(msg) = seeder.conn.recv() else {
          continue;
        };

        let Ok((_, payload)) = msg.parse_extended_payload() else {
          continue;
        };

        let req: extended::UtMetadata = match Message::from_bencode(payload) {
          Ok(req) => req,
          Err(_) => continue,
        };
        if req.piece > pieces {
          continue;
        }

        let range = std::ops::Range {
          start: extended::UtMetadata::PIECE_LENGTH * req.piece,
          end: if pieces == 1 {
            info_dict.len()
          } else {
            extended::UtMetadata::PIECE_LENGTH * (req.piece + 1)
          },
        };

        seeder
          .send_ut_metadata_data(req.piece, info_dict.len(), &info_dict[range])
          .unwrap();
      }
    });

    (seeder, addr)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn new_one_piece_info() -> Info {
    Info {
      private: Some(true),
      piece_length: Bytes(9001),
      name: "foo".into(),
      source: None,
      pieces: PieceList::new(),
      mode: Mode::Single {
        md5sum: None,
        length: Bytes(1),
      },
      update_url: None,
    }
  }

  fn new_two_piece_info() -> Info {
    Info {
      private: Some(true),
      piece_length: Bytes(9001),
      name: "a".repeat(extended::UtMetadata::PIECE_LENGTH),
      source: None,
      pieces: PieceList::from_pieces(["hello", "cargo", "test"]),
      mode: Mode::Single {
        md5sum: None,
        length: Bytes(1),
      },
      update_url: None,
    }
  }

  /// Start a remote client that listens on the returned `SocketAddr`. The client responds to the
  /// first received handshake. The client performs `work` if the handshake is successful.
  fn spawn_peer<W, T>(infohash: Infohash, work: W) -> (thread::JoinHandle<Result<T>>, SocketAddr)
  where
    W: Fn(Client) -> Result<T> + Send + 'static,
    T: Send + 'static,
  {
    let listener = TcpListener::bind((Ipv4Addr::UNSPECIFIED, 0)).unwrap();
    let addr = listener.local_addr().unwrap();
    let join_handle = thread::spawn(move || work(Client::listen(&listener, infohash)?));
    (join_handle, (Ipv4Addr::LOCALHOST, addr.port()).into())
  }

  fn spawn_idle_peer(infohash: Infohash) -> (thread::JoinHandle<Result<()>>, SocketAddr) {
    spawn_peer(infohash, |_| Ok(()))
  }

  fn spawn_info_dict_fetcher(infohash: Infohash) -> (thread::JoinHandle<Result<Info>>, SocketAddr) {
    spawn_peer(infohash, Client::fetch_info_dict)
  }

  fn new_client_ready_to_send_ut_metadata_data(
    addr: SocketAddr,
    infohash: Infohash,
    info: Info,
  ) -> peer::Client {
    let mut c = Client::connect(&addr, infohash).unwrap();
    c.info = Some(info);
    c.send_extension_handshake().unwrap();
    expect_extended_handshake(&mut c);
    expect_ut_metadata_request(&mut c, 0);
    c
  }

  fn expect_extended_handshake(c: &mut Client) {
    let msg = c.conn.recv().unwrap();
    assert_eq!(msg.flavour, message::Flavour::Extended);
    let (id, _) = msg.parse_extended_payload().unwrap();
    assert_eq!(id, extended::Id::Handshake);
    c.handle_msg(&msg).unwrap();
    assert_matches!(&c.extension_handshake, Some(..));
    assert_matches!(c.ut_metadata_msg_id(), Ok(..));
  }

  fn expect_ut_metadata_request(c: &mut Client, piece: usize) {
    let msg = c.conn.recv().unwrap();
    assert_eq!(msg.flavour, message::Flavour::Extended);
    let (id, payload) = msg.parse_extended_payload().unwrap();
    assert_eq!(id, extended::Id::UtMetadata);
    let ut_metadata_request: extended::UtMetadata = Message::from_bencode(payload).unwrap();
    assert_eq!(
      ut_metadata_request.msg_type,
      u8::from(extended::ut_metadata::MsgType::Request)
    );
    assert_eq!(ut_metadata_request.piece, piece);
  }

  #[test]
  fn handshake() {
    let info = Info {
      private: Some(true),
      piece_length: Bytes(9001),
      name: "test info".into(),
      source: None,
      pieces: PieceList::new(),
      mode: Mode::Single {
        md5sum: None,
        length: Bytes(1),
      },
      update_url: None,
    };
    let infohash = info.infohash_lossy().unwrap();

    let (remote_handle, addr) = spawn_idle_peer(infohash);
    assert_matches!(Client::connect(&addr, infohash), Ok(..));
    assert_matches!(remote_handle.join(), Ok(..));
  }

  #[test]
  fn handshake_bad_bt_header() {
    let info = new_one_piece_info();
    let infohash = info.infohash_lossy().unwrap();
    let (handle, addr) = spawn_idle_peer(infohash);

    let mut stream = TcpStream::connect_timeout(&addr, Duration::new(3, 0)).unwrap();
    let mut payload = peer::handshake::Handshake::new(infohash).serialize();
    // Deface the handshake.
    payload[0] = b'i';
    payload[1] = b'm';
    payload[2] = b'd';
    payload[3] = b'l';
    stream.write_all(&payload[..]).unwrap();

    assert_matches!(handle.join().unwrap(), Err(Error::PeerHandshakeHeader));
  }

  #[test]
  fn handshake_infohash_mismatch() {
    let info = new_one_piece_info();
    let mut info2 = new_one_piece_info();
    info2.name = String::from("bar");
    let infohash = info.infohash_lossy().unwrap();
    let infohash2 = info2.infohash_lossy().unwrap();
    let (handle, addr) = spawn_idle_peer(infohash);

    assert_matches!(
      Client::connect(&addr, infohash2),
      Err(Error::Network { .. })
    );
    assert_matches!(handle.join().unwrap(), Err(Error::PeerHandshakeInfohash));
  }

  #[test]
  #[ignore]
  fn connection_timeout() {
    let info = new_one_piece_info();
    let infohash = info.infohash_lossy().unwrap();
    let listener = TcpListener::bind((Ipv4Addr::UNSPECIFIED, 0)).unwrap();
    let addr = (Ipv4Addr::LOCALHOST, listener.local_addr().unwrap().port()).into();

    assert_matches!(Client::connect(&addr, infohash), Err(Error::Network { .. }));
  }

  #[test]
  fn extension_handshake() {
    let info = new_one_piece_info();
    let infohash = info.infohash_lossy().unwrap();

    let (handle, addr) = spawn_peer(infohash, |mut c| {
      c.send_extension_handshake()?;
      let msg = c.conn.recv()?;
      c.handle_msg(&msg)
    });

    let mut c = Client::connect(&addr, infohash).unwrap();
    c.send_extension_handshake().unwrap();
    c.conn.recv().and_then(|msg| c.handle_msg(&msg)).unwrap();
    let handshake = c.extension_handshake.unwrap();
    assert_eq!(
      handshake.version.unwrap(),
      format!("intermodal {}", consts::VERSION)
    );
    assert_matches!(handle.join().unwrap(), Ok(()));
  }

  #[test]
  fn extension_handshake_errors() {
    let info = new_one_piece_info();
    let infohash = info.infohash_lossy().unwrap();
    let (handle, addr) = spawn_peer(infohash, |mut c| {
      let msg = c.conn.recv()?;
      c.handle_msg(&msg)?;
      c.send_extension_handshake()
    });

    let mut local = Client::connect(&addr, infohash).unwrap();
    assert_matches!(
      local.ut_metadata_size(),
      Err(Error::PeerNoExtendedHandshake)
    );
    assert_matches!(
      local.ut_metadata_msg_id(),
      Err(Error::PeerNoExtendedHandshake)
    );
    local.send_extension_handshake().unwrap();
    local
      .conn
      .recv()
      .and_then(|msg| local.handle_msg(&msg))
      .unwrap();
    assert_matches!(
      local.ut_metadata_size(),
      Err(Error::PeerUtMetadataMetadataSizeNotKnown)
    );
    assert_matches!(handle.join().unwrap(), Ok(()));
  }

  #[test]
  fn ut_metadata_not_supported() {
    let info = new_one_piece_info();
    let infohash = info.infohash_lossy().unwrap();
    let (join_handle, addr) = spawn_info_dict_fetcher(infohash);
    let mut c = Client::connect(&addr, infohash).unwrap();
    let mut extended_handshake = extended::Handshake::new(); // no ut_metadata message id set
    extended_handshake.metadata_size = Some(1);
    c.conn
      .send(&Message::new_extended(extended::Id::Handshake.into(), extended_handshake).unwrap())
      .unwrap();

    assert_matches!(
      join_handle.join().unwrap(),
      Err(Error::PeerUtMetadataNotSupported)
    );
  }

  #[test]
  fn metadata_size_not_known() {
    let info = new_one_piece_info();
    let infohash = info.infohash_lossy().unwrap();

    let (join_handle, addr) = spawn_info_dict_fetcher(infohash);
    let mut c = Client::connect(&addr, infohash).unwrap();
    c.send_extension_handshake().unwrap();

    assert_matches!(
      join_handle.join().unwrap(),
      Err(Error::PeerUtMetadataMetadataSizeNotKnown)
    );
  }

  #[test]
  fn fetch_info_one_piece() {
    let info = new_one_piece_info();
    let info_dict = bendy::serde::ser::to_bytes(&info).unwrap();
    let infohash = info.infohash_lossy().unwrap();

    let (join_handle, addr) = spawn_info_dict_fetcher(infohash);
    let mut c = new_client_ready_to_send_ut_metadata_data(addr, infohash, info.clone());

    c.send_ut_metadata_data(0, info_dict.len(), &info_dict[..])
      .unwrap();

    assert_eq!(join_handle.join().unwrap().unwrap(), info);
  }

  #[test]
  fn fetch_info_two_pieces() {
    let info = new_two_piece_info();
    let info_dict = bendy::serde::ser::to_bytes(&info).unwrap();
    let infohash = info.infohash_lossy().unwrap();
    let (fetcher, addr) = spawn_info_dict_fetcher(infohash);
    let mut c = new_client_ready_to_send_ut_metadata_data(addr, infohash, info.clone());

    c.send_ut_metadata_data(
      0,
      info_dict.len(),
      &info_dict[..extended::UtMetadata::PIECE_LENGTH],
    )
    .unwrap();
    expect_ut_metadata_request(&mut c, 1);
    c.send_ut_metadata_data(
      1,
      info_dict.len(),
      &info_dict[extended::UtMetadata::PIECE_LENGTH..],
    )
    .unwrap();

    assert_eq!(fetcher.join().unwrap().unwrap(), info);
  }

  #[test]
  fn ut_metadata_wrong_piece_length() {
    let info = new_two_piece_info();
    let info_dict = bendy::serde::ser::to_bytes(&info).unwrap();
    let infohash = info.infohash_lossy().unwrap();

    let (join_handle, addr) = spawn_info_dict_fetcher(infohash);
    let mut c = new_client_ready_to_send_ut_metadata_data(addr, infohash, info);

    c.send_ut_metadata_data(0, info_dict.len(), &info_dict[..])
      .unwrap();

    assert_matches!(
      join_handle.join().unwrap(),
      Err(Error::PeerUtMetadataPieceLength)
    );
  }

  #[test]
  fn ut_metadata_receive_wrong_piece() {
    let info = new_two_piece_info();
    let info_dict = bendy::serde::ser::to_bytes(&info).unwrap();
    let infohash = info.infohash_lossy().unwrap();

    let (join_handle, addr) = spawn_info_dict_fetcher(infohash);
    let mut c = new_client_ready_to_send_ut_metadata_data(addr, infohash, info);
    c.send_ut_metadata_data(
      1, // wrong piece
      info_dict.len(),
      &info_dict[extended::UtMetadata::PIECE_LENGTH..],
    )
    .unwrap();

    assert_matches!(
      join_handle.join().unwrap(),
      Err(Error::PeerUtMetadataWrongPiece)
    );
  }

  #[test]
  fn receive_info_dict_with_wrong_infohash() {
    let info = new_one_piece_info();
    let mut wrong_info = new_one_piece_info();
    wrong_info.name = String::from("bar");
    let infohash = info.infohash_lossy().unwrap();

    let (join_handle, addr) = spawn_info_dict_fetcher(infohash);
    let mut c = new_client_ready_to_send_ut_metadata_data(addr, infohash, info);

    let wrong_info_dict = bendy::serde::ser::to_bytes(&wrong_info).unwrap();
    c.send_ut_metadata_data(0, wrong_info_dict.len(), &wrong_info_dict[..])
      .expect("ut_metadata data send succeeds");

    assert_matches!(
      join_handle.join().unwrap(),
      Err(Error::PeerUtMetadataWrongInfohash)
    );
  }

  #[test]
  fn receive_info_dict_that_fails_to_deserialize() {
    let info = new_one_piece_info();
    let infohash = info.infohash_lossy().unwrap();
    let info_dict_len = bendy::serde::ser::to_bytes(&info).unwrap().len();
    let wrong_info_dict = vec![0; info_dict_len];

    let (join_handle, addr) = spawn_info_dict_fetcher(infohash);
    let mut c = new_client_ready_to_send_ut_metadata_data(addr, infohash, info);
    c.send_ut_metadata_data(0, wrong_info_dict.len(), &wrong_info_dict[..])
      .unwrap();

    assert_matches!(
      join_handle.join().unwrap(),
      Err(Error::PeerUtMetadataInfoDeserialize { .. })
    );
  }

  #[test]
  fn receive_info_dict_with_wrong_metadata_size() {
    let info = new_one_piece_info();
    let infohash = info.infohash_lossy().unwrap();
    let info_dict_len = bendy::serde::ser::to_bytes(&info).unwrap().len();
    let wrong_info_dict = vec![0; info_dict_len + 1];

    let (join_handle, addr) = spawn_info_dict_fetcher(infohash);
    let mut c = new_client_ready_to_send_ut_metadata_data(addr, infohash, info);
    c.send_ut_metadata_data(0, wrong_info_dict.len(), &wrong_info_dict[..])
      .unwrap();

    assert_matches!(
      join_handle.join().unwrap(),
      Err(Error::PeerUtMetadataInfoLength { .. })
    );
  }
}
