use crate::common::*;

#[derive(PartialEq, Debug, Clone)]
pub(crate) enum InputTarget {
  Path(PathBuf),
  Stdin,
}

impl InputTarget {
  pub(crate) fn try_from_os_str(text: &OsStr) -> Result<Self, OsString> {
    text
      .try_into()
      .map_err(|err: Error| OsString::from(err.to_string()))
  }
}

impl TryFrom<&OsStr> for InputTarget {
  type Error = Error;

  fn try_from(text: &OsStr) -> Result<Self, Self::Error> {
    if text.is_empty() {
      return Err(Error::InputTargetEmpty);
    };

    if text == OsStr::new("-") {
      Ok(Self::Stdin)
    } else {
      Ok(Self::Path(text.into()))
    }
  }
}

impl Display for InputTarget {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match self {
      Self::Stdin => write!(f, "standard input"),
      Self::Path(path) => write!(f, "`{}`", path.display()),
    }
  }
}

impl<P: AsRef<Path>> PartialEq<P> for InputTarget {
  fn eq(&self, other: &P) -> bool {
    match self {
      Self::Path(path) => path == other.as_ref(),
      Self::Stdin => Path::new("-") == other.as_ref(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn file() {
    assert_eq!(
      InputTarget::try_from(OsStr::new("foo")).unwrap(),
      InputTarget::Path("foo".into()),
    );
  }

  #[test]
  fn stdio() {
    assert_eq!(
      InputTarget::try_from(OsStr::new("-")).unwrap(),
      InputTarget::Stdin
    );
  }

  #[test]
  fn display_file() {
    let path = PathBuf::from("./path");
    let have = InputTarget::Path(path).to_string();
    let want = "`./path`";
    assert_eq!(have, want);
  }

  #[test]
  fn display_stdio() {
    let have = InputTarget::Stdin.to_string();
    let want = "standard input";
    assert_eq!(have, want);
  }
}
