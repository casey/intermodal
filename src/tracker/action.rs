#[allow(dead_code)]
#[derive(Debug)]
pub enum Action {
  Connect,
  Announce,
  Scrape,
  Error,
}

impl Into<u32> for Action {
  fn into(self) -> u32 {
    match self {
      Action::Connect => 0,
      Action::Announce => 1,
      Action::Scrape => 2,
      Action::Error => 3,
    }
  }
}
