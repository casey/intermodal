use crate::common::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, IntoStaticStr, EnumString)]
#[strum(serialize_all = "kebab-case")]
pub(crate) enum SortOrder {
  #[default]
  Ascending,
  Descending,
}

impl SortOrder {
  pub(crate) fn name(self) -> &'static str {
    self.into()
  }
}
