pub(crate) trait Slug {
  fn slug(&self) -> String;
}

impl Slug for str {
  fn slug(&self) -> String {
    self.replace(' ', "-").to_lowercase()
  }
}
