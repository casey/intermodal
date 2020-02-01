use crate::common::*;

#[derive(Eq, PartialEq, Debug)]
pub(crate) enum Lint {
  UnevenPieceLength,
  SmallPieceLength,
}

// impl Lint {
//   fn all() -> &'static [Lint] {
//     &[Self::UnevenPieceLength]
//   }
// }

impl FromStr for Lint {
  type Err = Error;

  fn from_str(text: &str) -> Result<Self, Self::Err> {
    match text.replace('_', "-").to_lowercase().as_str() {
      "uneven-piece-length" => Ok(Self::UnevenPieceLength),
      "small-piece-length" => Ok(Self::SmallPieceLength),
      _ => Err(Error::LintUnknown {
        text: text.to_string(),
      }),
    }
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
