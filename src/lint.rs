use crate::common::*;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Ord, PartialOrd)]
pub(crate) enum Lint {
  UnevenPieceLength,
  SmallPieceLength,
}

const UNEVEN_PIECE_LENGTH: &str = "uneven-piece-length";
const SMALL_PIECE_LENGTH: &str = "small-piece-length";

impl Lint {
  pub(crate) fn name(self) -> &'static str {
    match self {
      Self::UnevenPieceLength => UNEVEN_PIECE_LENGTH,
      Self::SmallPieceLength => SMALL_PIECE_LENGTH,
    }
  }
}

impl FromStr for Lint {
  type Err = Error;

  fn from_str(text: &str) -> Result<Self, Self::Err> {
    match text.replace('_', "-").to_lowercase().as_str() {
      UNEVEN_PIECE_LENGTH => Ok(Self::UnevenPieceLength),
      SMALL_PIECE_LENGTH => Ok(Self::SmallPieceLength),
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
