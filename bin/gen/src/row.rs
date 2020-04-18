pub(crate) trait Row {
  fn header() -> &'static [&'static str];

  fn entries(&self) -> Vec<&str>;
}
