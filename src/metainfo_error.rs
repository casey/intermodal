use crate::common::*;

#[derive(Debug, Copy, Clone)]
pub(crate) enum MetainfoError {
  Type,
  InfoMissing,
  InfoType,
}

impl MetainfoError {
  fn message(self) -> &'static str {
    match self {
      Self::Type => "Top-level value not dictionary",
      Self::InfoMissing => "Dictionary missing info key",
      Self::InfoType => "Info value not dictionary",
    }
  }
}

impl Display for MetainfoError {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{}", self.message())
  }
}

impl std::error::Error for MetainfoError {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn display() {
    assert_eq!(
      MetainfoError::Type.to_string(),
      "Top-level value not dictionary"
    );

    assert_eq!(
      MetainfoError::InfoMissing.to_string(),
      "Dictionary missing info key",
    );

    assert_eq!(
      MetainfoError::InfoType.to_string(),
      "Info value not dictionary",
    );
  }
}
