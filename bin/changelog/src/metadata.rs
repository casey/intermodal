use crate::common::*;

#[derive(Deserialize, Serialize)]
pub(crate) struct Metadata {
  #[serde(default, rename = "type")]
  pub(crate) kind: Option<Kind>,
  #[serde(default)]
  pub(crate) pr: Option<Url>,
  #[serde(default)]
  pub(crate) fixes: Vec<Url>,
}

impl Metadata {
  #[throws(as Option)]
  pub(crate) fn from_commit(commit: &Commit) -> Metadata {
    const BLANK: &str = "\n\n";

    let message = String::from_utf8_lossy(commit.message_bytes());

    let blank = message.rfind(BLANK)?;

    let yaml = &message[blank + BLANK.len()..];

    let metadata = serde_yaml::from_str(yaml).ok()?;

    metadata
  }

  pub(crate) fn emoji(&self) -> &'static str {
    if let Some(kind) = self.kind {
      match kind {
        Kind::Added => ":sparkles:",
        Kind::Breaking => ":boom:",
        Kind::Changed => ":zap:",
        Kind::Development => ":wrench:",
        Kind::Distribution => ":package:",
        Kind::Documentation => ":books:",
        Kind::Fixed => ":bug:",
        Kind::Reform => ":art:",
        Kind::Release => ":bookmark:",
        Kind::Testing => ":white_check_mark:",
      }
    } else {
      ":construction:"
    }
  }
}

impl Default for Metadata {
  fn default() -> Metadata {
    Metadata {
      kind: None,
      pr: None,
      fixes: Vec::new(),
    }
  }
}

impl Display for Metadata {
  #[throws(fmt::Error)]
  fn fmt(&self, f: &mut Formatter) {
    writeln!(f)?;
    writeln!(
      f,
      "{}",
      serde_yaml::to_string(&self)
        .unwrap()
        .split("---")
        .last()
        .unwrap()
        .trim()
    )?;
  }
}
