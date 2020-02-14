struct HostPort {
  host: Host,
  port: u16,
}

impl FromStr for HostPort {
  type Err = Error;

  fn from_str(text: &str) -> Result<Self, Self::Err> {
    let socket_address_re = Regex::new(
      r"(?x)
      ^
      (?P<host>.*?)
      :
      (?P<port>\d+?)
      $
      ",
    )
    .unwrap();

    if let Some(captures) = socket_address_re.captures(text) {
      let host_text = captures.name("host").unwrap().as_str();
      let port_text = captures.name("port").unwrap().as_str();

      let host = Host::parse(host_text).map_err(|_| Error::NodeParse {
        text: text.to_owned(),
      })?;
      let port = port_text.parse::<u16>().map_err(|_| Error::NodeParse {
        text: text.to_owned(),
      })?;

      Ok(SocketAddress { host, port })
    } else {
      Err(Error::NodeParse {
        text: text.to_owned(),
      })
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse() {
    todo!()
  }

  #[test]
  fn serialize() {
    todo!()
  }

  #[test]
  fn deserialize() {
    todo!()
  }
}
