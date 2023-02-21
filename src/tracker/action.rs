#[derive(Debug)]
pub enum Action {
  Connect,
  Announce,
  Scrape,
  Unsupported,
}

impl From<Action> for u32 {
  fn from(a: Action) -> Self {
    match a {
      Action::Connect => 0,
      Action::Announce => 1,
      Action::Scrape => 2,
      Action::Unsupported => 0xffff,
    }
  }
}

impl From<u32> for Action {
  fn from(x: u32) -> Self {
    match x {
      0 => Action::Connect,
      1 => Action::Announce,
      2 => Action::Scrape,
      _ => Action::Unsupported,
    }
  }
}
