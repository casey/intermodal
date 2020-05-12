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
  Removed,
}

impl Kind {
  pub(crate) fn emoji_character(self) -> &'static str {
    match self {
      Self::Added => "✨",
      Self::Breaking => "💥",
      Self::Changed => "⚡️",
      Self::Dependencies => "⬆️",
      Self::Development => "🔧",
      Self::Distribution => "📦",
      Self::Documentation => "📚",
      Self::Fixed => "🐛",
      Self::Reform => "🎨",
      Self::Release => "🔖",
      Self::Testing => "✅",
      Self::Removed => "➖",
    }
  }

  pub(crate) fn emoji_name(self) -> &'static str {
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
      Self::Removed => ":heavy_minus_sign:",
    }
  }
}
