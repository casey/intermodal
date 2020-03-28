use crate::common::*;

#[derive(
  Eq, PartialEq, Debug, Copy, Clone, Ord, PartialOrd, EnumVariantNames, IntoStaticStr, EnumString,
)]
#[strum(serialize_all = "kebab-case")]
pub(crate) enum Lint {
  PrivateTrackerless,
  SmallPieceLength,
  UnevenPieceLength,
}

impl Lint {
  pub(crate) fn name(self) -> &'static str {
    self.into()
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
  fn variants() {
    assert_eq!(
      Lint::VARIANTS,
      &[
        "private-trackerless",
        "small-piece-length",
        "uneven-piece-length"
      ]
    );
  }

  #[test]
  fn from_str_ok() {
    assert_eq!(
      Lint::UnevenPieceLength,
      "uneven-piece-length".parse().unwrap()
    );
  }

  #[test]
  fn convert() {
    fn case(text: &str, value: Lint) {
      assert_eq!(value, text.parse().unwrap());
      assert_eq!(value.name(), text);
      assert_eq!(value.to_string(), value.name());
    }

    case("private-trackerless", Lint::PrivateTrackerless);
    case("small-piece-length", Lint::SmallPieceLength);
    case("uneven-piece-length", Lint::UnevenPieceLength);
  }

  #[test]
  fn from_str_err() {
    assert_matches!(
      "foo".parse::<Lint>(),
      Err(strum::ParseError::VariantNotFound)
    );
  }
}
