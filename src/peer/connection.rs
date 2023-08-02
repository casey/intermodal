use crate::common::*;

use message::Message;
use peer::handshake::Handshake;
use peer::message;

#[derive(Debug)]
pub struct Connection {
  pub(crate) stream: TcpStream,
  pub(crate) handshake: Handshake,
}

impl Connection {
  pub(crate) fn new(addr: &SocketAddr, infohash: Infohash) -> Result<Self> {
    let mut stream =
      TcpStream::connect_timeout(addr, Duration::new(3, 0)).context(error::Network)?;
    stream
      .set_read_timeout(Some(Duration::new(3, 0)))
      .context(error::Network)?;

    Self::send_handshake(&mut stream, infohash)?;
    let handshake = Self::recv_handshake(&mut stream, infohash)?;

    Ok(Self { stream, handshake })
  }

  fn recv_handshake(stream: &mut TcpStream, infohash: Infohash) -> Result<Handshake> {
    let mut buf = [0u8; Handshake::LENGTH];
    stream.read_exact(&mut buf).context(error::Network)?;
    let handshake = Handshake::try_from(buf)?;
    if Infohash::from(handshake.infohash) != infohash {
      return Err(error::Error::PeerHandshakeInfohash);
    }
    Ok(handshake)
  }

  fn send_handshake(stream: &mut TcpStream, infohash: Infohash) -> Result<Handshake> {
    let handshake = Handshake::new(infohash);
    stream
      .write_all(&handshake.serialize()[..])
      .context(error::Network)?;
    Ok(handshake)
  }

  pub(crate) fn recv(&mut self) -> Result<Message> {
    // The message header is four bytes of message length, plus a one byte instruction.
    let mut header = [0u8; 5];
    self
      .stream
      .read_exact(&mut header)
      .context(error::Network)?;

    let length = u32::from_be_bytes(
      header[..4]
        .try_into()
        .invariant_unwrap("bound is checked by read_exact and the length of buf"),
    );

    let payload = if length > 1 {
      let mut payload = Vec::new();
      (&self.stream)
        .take((length - 1).into())
        .read_to_end(&mut payload)
        .context(error::Network)?;
      Some(payload)
    } else {
      None
    };

    Ok(Message {
      flavour: message::Flavour::from(header[4]),
      payload,
    })
  }

  pub(crate) fn send(&mut self, msg: &message::Message) -> Result<()> {
    self
      .stream
      .write_all(&msg.serialize()?)
      .context(error::Network)
  }

  pub(crate) fn supports_extension_protocol(&self) -> bool {
    self.handshake.supports_extension_protocol()
  }

  #[cfg(test)]
  pub(crate) fn from(mut stream: TcpStream, infohash: Infohash) -> Result<Self> {
    let handshake = Self::recv_handshake(&mut stream, infohash)?;
    Self::send_handshake(&mut stream, infohash)?;
    Ok(Self { stream, handshake })
  }
}
