use crate::common::*;

#[derive(Clone, Copy, Debug, PartialEq, IntoStaticStr, EnumString)]
#[strum(serialize_all = "kebab-case")]
pub(crate) enum SortKey {
  Path,
  Size,
}

impl SortKey {
  pub(crate) fn name(self) -> &'static str {
    self.into()
  }
}
