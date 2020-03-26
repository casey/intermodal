use crate::common::*;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Ord, PartialOrd)]
pub(crate) enum FileOrder {
  AlphabeticalDesc,
  AlphabeticalAsc,
  SizeDesc,
  SizeAsc,
}

impl FileOrder {
  pub(crate) const ALPHABETICAL_ASC: &'static str = "alphabetical-asc";
  pub(crate) const ALPHABETICAL_DESC: &'static str = "alphabetical-desc";
  pub(crate) const SIZE_ASC: &'static str = "size-asc";
  pub(crate) const SIZE_DESC: &'static str = "size-desc";
  pub(crate) const VALUES: &'static [&'static str] = &[
    Self::ALPHABETICAL_DESC,
    Self::ALPHABETICAL_ASC,
    Self::SIZE_DESC,
    Self::SIZE_ASC,
  ];

  pub(crate) fn name(self) -> &'static str {
    match self {
      Self::AlphabeticalDesc => Self::ALPHABETICAL_DESC,
      Self::AlphabeticalAsc => Self::ALPHABETICAL_ASC,
      Self::SizeDesc => Self::SIZE_DESC,
      Self::SizeAsc => Self::SIZE_ASC,
    }
  }

  pub(crate) fn compare_file_info(self, a: &FileInfo, b: &FileInfo) -> Ordering {
    match self {
      Self::AlphabeticalAsc => a.path.cmp(&b.path),
      Self::AlphabeticalDesc => a.path.cmp(&b.path).reverse(),
      Self::SizeAsc => a.length.cmp(&b.length).then_with(|| a.path.cmp(&b.path)),
      Self::SizeDesc => a.length.cmp(&b.length).reverse().then_with(|| a.path.cmp(&b.path)),
    }
  }
}

impl FromStr for FileOrder {
  type Err = Error;

  fn from_str(text: &str) -> Result<Self, Self::Err> {
    match text.replace('_', "-").to_lowercase().as_str() {
      Self::ALPHABETICAL_DESC => Ok(Self::AlphabeticalDesc),
      Self::ALPHABETICAL_ASC => Ok(Self::AlphabeticalAsc),
      Self::SIZE_DESC => Ok(Self::SizeDesc),
      Self::SIZE_ASC => Ok(Self::SizeAsc),
      _ => Err(Error::FileOrderUnknown {
        text: text.to_string(),
      }),
    }
  }
}

impl Display for FileOrder {
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
      FileOrder::AlphabeticalDesc,
      "alphabetical_desc".parse().unwrap()
    );

    assert_eq!(
      FileOrder::AlphabeticalDesc,
      "alphabetical-desc".parse().unwrap()
    );

    assert_eq!(
      FileOrder::AlphabeticalDesc,
      "ALPHABETICAL-desc".parse().unwrap()
    );
  }

  #[test]
  fn convert() {
    fn case(text: &str, value: FileOrder) {
      assert_eq!(value, text.parse().unwrap());
      assert_eq!(value.name(), text);
    }

    case("alphabetical-desc", FileOrder::AlphabeticalDesc);
    case("alphabetical-asc", FileOrder::AlphabeticalAsc);
    case("size-desc", FileOrder::SizeDesc);
    case("size-asc", FileOrder::SizeAsc);
  }

  #[test]
  fn from_str_err() {
    assert_matches!(
      "foo".parse::<FileOrder>(),
      Err(Error::FileOrderUnknown { text }) if text == "foo"
    );
  }
}
