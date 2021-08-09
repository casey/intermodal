#[derive(Debug)]
pub enum Action {
  Connect,
  Announce,
  Scrape,
  Error,
}

impl From<Action> for u32 {
  fn from(a: Action) -> Self {
    match a {
      Action::Connect => 0,
      Action::Announce => 1,
      Action::Scrape => 2,
      Action::Error => 3,
    }
  }
}
