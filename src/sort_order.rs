use crate::common::*;

#[derive(Clone, Copy, Debug, PartialEq, IntoStaticStr, EnumString)]
#[strum(serialize_all = "kebab-case")]
pub(crate) enum SortOrder {
  Ascending,
  Descending,
}

impl SortOrder {
  pub(crate) fn name(self) -> &'static str {
    self.into()
  }
}

impl Default for SortOrder {
  fn default() -> Self {
    Self::Ascending
  }
}
