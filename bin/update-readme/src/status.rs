use crate::common::*;

pub(crate) enum Status {
  Unknown,
  NotApplicable,
  Supported,
  NotSupported { tracking_issue: Option<u64> },
}

impl FromStr for Status {
  type Err = String;

  fn from_str(text: &str) -> Result<Self, Self::Err> {
    let error = || format!("invalid status: {}", text);

    let unescaped = text.replace('\\', "");

    let (emoji, tracking_issue) = if !unescaped.starts_with('[') {
      (text, None)
    } else {
      let status_pattern = Regex::new(
        r"(?x)
        ^
        \[
          (?P<emoji>:[a-zA-Z0-9]+:)
        \]
        \(
          https://github.com/casey/intermodal/issues/(?P<tracking_issue>[0-9]+)
        \)
        $
      ",
      )
      .unwrap();

      let captures = status_pattern.captures(&unescaped).ok_or_else(error)?;

      let emoji = captures.name("emoji").unwrap().as_str();

      let tracking_issue = captures
        .name("tracking_issue")
        .map(|text| text.as_str().parse::<u64>().unwrap());

      (emoji, tracking_issue)
    };

    match emoji {
      "x" => Ok(Status::NotSupported { tracking_issue }),
      "+" => Ok(Status::Supported),
      "-" => Ok(Status::NotApplicable),
      "?" => Ok(Status::Unknown),
      ":x:" => Ok(Status::NotSupported { tracking_issue }),
      ":white_check_mark:" => Ok(Status::Supported),
      ":heavy_minus_sign:" => Ok(Status::NotApplicable),
      ":grey_question:" => Ok(Status::Unknown),
      _ => Err(error()),
    }
  }
}

impl Display for Status {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match self {
      Self::Unknown => write!(f, ":grey_question:"),
      Self::NotApplicable => write!(f, ":heavy_minus_sign:"),
      Self::Supported => write!(f, ":white_check_mark:"),
      Self::NotSupported {
        tracking_issue: None,
      } => write!(f, ":x:"),
      Self::NotSupported {
        tracking_issue: Some(number),
      } => write!(
        f,
        "[:x:](https://github.com/casey/intermodal/issues/{})",
        number
      ),
    }
  }
}
