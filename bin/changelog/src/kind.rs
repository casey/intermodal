use crate::common::*;

#[derive(
  Deserialize, Serialize, Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, IntoStaticStr,
)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum Kind {
  Added,
  Breaking,
  Changed,
  Development,
  Distribution,
  Documentation,
  Fixed,
  Reform,
  Release,
  Testing,
}
