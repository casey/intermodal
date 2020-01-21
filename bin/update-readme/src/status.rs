use crate::common::*;

pub(crate) enum Status {
  Unknown,
  NotApplicable,
  Supported,
  NotSupported,
}

impl FromStr for Status {
  type Err = String;

  fn from_str(text: &str) -> Result<Self, Self::Err> {
    match text.replace('\\', "").as_str() {
      "x" => Ok(Self::NotSupported),
      "+" => Ok(Self::Supported),
      "-" => Ok(Self::NotApplicable),
      "?" => Ok(Self::Unknown),
      ":x:" => Ok(Self::NotSupported),
      ":white_check_mark:" => Ok(Self::Supported),
      ":heavy_minus_sign:" => Ok(Self::NotApplicable),
      ":grey_question:" => Ok(Self::Unknown),
      _ => Err(format!("invalid status: {}", text)),
    }
  }
}

impl Display for Status {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match self {
      Self::Unknown => write!(f, ":grey_question:"),
      Self::NotApplicable => write!(f, ":heavy_minus_sign:"),
      Self::Supported => write!(f, ":white_check_mark:"),
      Self::NotSupported => write!(f, ":x:"),
    }
  }
}
