use crate::common::*;

#[derive(PartialEq, Debug, Clone)]
pub(crate) enum InputTarget {
  File(PathBuf),
  Stdin,
}

impl From<&OsStr> for InputTarget {
  fn from(text: &OsStr) -> Self {
    if text == OsStr::new("-") {
      Self::Stdin
    } else {
      Self::File(text.into())
    }
  }
}

impl Display for InputTarget {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match self {
      Self::Stdin => write!(f, "standard input"),
      Self::File(path) => write!(f, "`{}`", path.display()),
    }
  }
}

#[cfg(test)]
impl<P: AsRef<Path>> PartialEq<P> for InputTarget {
  fn eq(&self, other: &P) -> bool {
    match self {
      Self::File(path) => path == other.as_ref(),
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
      InputTarget::from(OsStr::new("foo")),
      InputTarget::File("foo".into())
    );
  }

  #[test]
  fn stdio() {
    assert_eq!(InputTarget::from(OsStr::new("-")), InputTarget::Stdin);
  }

  #[test]
  fn display_file() {
    let path = PathBuf::from("./path");
    let have = InputTarget::File(path).to_string();
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
