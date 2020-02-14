use crate::common::*;

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Node {
  host: Host,
  port: u16,
}

impl FromStr for Node {
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

      let host = Host::parse(&host_text).context(error::NodeParseHost {
        text: text.to_owned(),
      })?;

      let port = port_text.parse::<u16>().context(error::NodeParsePort {
        text: text.to_owned(),
      })?;

      Ok(Self { host, port })
    } else {
      Err(Error::NodeParsePortMissing {
        text: text.to_owned(),
      })
    }
  }
}

impl Display for Node {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{}:{}", self.host, self.port)
  }
}

#[derive(Serialize, Deserialize)]
struct Tuple(String, u16);

impl From<&Node> for Tuple {
  fn from(node: &Node) -> Self {
    let host = match &node.host {
      Host::Domain(domain) => domain.to_string(),
      Host::Ipv4(ipv4) => ipv4.to_string(),
      Host::Ipv6(ipv6) => ipv6.to_string(),
    };
    Self(host, node.port)
  }
}

impl Serialize for Node {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    Tuple::from(self).serialize(serializer)
  }
}

impl<'de> Deserialize<'de> for Node {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let tuple = Tuple::deserialize(deserializer)?;

    let host = if tuple.0.contains(':') {
      Host::parse(&format!("[{}]", tuple.0))
    } else {
      Host::parse(&tuple.0)
    }
    .map_err(|error| D::Error::custom(format!("Failed to parse node host: {}", error)))?;

    Ok(Node {
      host,
      port: tuple.1,
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use std::net::{Ipv4Addr, Ipv6Addr};

  fn case(host: Host, port: u16, text: &str, bencode: &str) {
    let node = Node { host, port };
    let parsed: Node = text.parse().expect(&format!("Failed to parse {}", text));
    assert_eq!(parsed, node);
    let ser = bendy::serde::to_bytes(&node).unwrap();
    assert_eq!(
      ser,
      bencode.as_bytes(),
      "Unexpected serialization: {} != {}",
      String::from_utf8_lossy(&ser),
      bencode,
    );
    let de = bendy::serde::from_bytes::<Node>(&ser).unwrap();
    assert_eq!(de, node);
  }

  #[test]
  fn test_domain() {
    case(
      Host::Domain("imdl.com".to_owned()),
      12,
      "imdl.com:12",
      "l8:imdl.comi12ee",
    );
  }

  #[test]
  fn test_ipv4() {
    case(
      Host::Ipv4(Ipv4Addr::new(1, 2, 3, 4)),
      100,
      "1.2.3.4:100",
      "l7:1.2.3.4i100ee",
    );
  }

  #[test]
  fn test_ipv6() {
    case(
      Host::Ipv6(Ipv6Addr::new(
        0x1234, 0x5678, 0x9ABC, 0xDEF0, 0x1234, 0x5678, 0x9ABC, 0xDEF0,
      )),
      65000,
      "[1234:5678:9abc:def0:1234:5678:9abc:def0]:65000",
      "l39:1234:5678:9abc:def0:1234:5678:9abc:def0i65000ee",
    );
  }
}
