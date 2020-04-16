use crate::common::*;

#[derive(
  Deserialize,
  Serialize,
  Debug,
  Clone,
  Copy,
  Eq,
  PartialEq,
  Ord,
  PartialOrd,
  IntoStaticStr,
  EnumVariantNames,
)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub(crate) enum Kind {
  Added,
  Breaking,
  Changed,
  Dependencies,
  Development,
  Distribution,
  Documentation,
  Fixed,
  Reform,
  Release,
  Testing,
}

impl Kind {
  pub(crate) fn emoji(self) -> &'static str {
    match self {
      Self::Added => ":sparkles:",
      Self::Breaking => ":boom:",
      Self::Changed => ":zap:",
      Self::Dependencies => ":arrow_up:",
      Self::Development => ":wrench:",
      Self::Distribution => ":package:",
      Self::Documentation => ":books:",
      Self::Fixed => ":bug:",
      Self::Reform => ":art:",
      Self::Release => ":bookmark:",
      Self::Testing => ":white_check_mark:",
    }
  }
}
