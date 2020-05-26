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
  Performance,
  Reform,
  Release,
  Removed,
  Testing,
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
      Self::Performance => "🐎",
      Self::Reform => "🎨",
      Self::Release => "🔖",
      Self::Removed => "➖",
      Self::Testing => "✅",
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
      Self::Performance => ":racehorse:",
      Self::Reform => ":art:",
      Self::Release => ":bookmark:",
      Self::Removed => ":heavy_minus_sign:",
      Self::Testing => ":white_check_mark:",
    }
  }
}
