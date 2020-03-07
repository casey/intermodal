use crate::common::*;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Ord, PartialOrd)]
pub(crate) enum Lint {
  UnevenPieceLength,
  SmallPieceLength,
}

impl Lint {
  const SMALL_PIECE_LENGTH: &'static str = "small-piece-length";
  const UNEVEN_PIECE_LENGTH: &'static str = "uneven-piece-length";
  pub(crate) const VALUES: &'static [&'static str] =
    &[Self::SMALL_PIECE_LENGTH, Self::UNEVEN_PIECE_LENGTH];

  pub(crate) fn name(self) -> &'static str {
    match self {
      Self::SmallPieceLength => Self::SMALL_PIECE_LENGTH,
      Self::UnevenPieceLength => Self::UNEVEN_PIECE_LENGTH,
    }
  }
}

impl FromStr for Lint {
  type Err = Error;

  fn from_str(text: &str) -> Result<Self, Self::Err> {
    match text.replace('_', "-").to_lowercase().as_str() {
      Self::SMALL_PIECE_LENGTH => Ok(Self::SmallPieceLength),
      Self::UNEVEN_PIECE_LENGTH => Ok(Self::UnevenPieceLength),
      _ => Err(Error::LintUnknown {
        text: text.to_string(),
      }),
    }
  }
}

impl Display for Lint {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{}", self.name())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn from_str_ok() {
    assert_eq!(
      Lint::UnevenPieceLength,
      "uneven_piece_length".parse().unwrap()
    );

    assert_eq!(
      Lint::UnevenPieceLength,
      "uneven-piece-length".parse().unwrap()
    );

    assert_eq!(
      Lint::UnevenPieceLength,
      "UNEVEN-piece-length".parse().unwrap()
    );
  }

  #[test]
  fn from_str_err() {
    assert_matches!(
      "foo".parse::<Lint>(),
      Err(Error::LintUnknown { text }) if text == "foo"
    );
  }
}
