use crate::common::*;

#[derive(PartialEq, Debug)]
pub(crate) enum OutputTarget {
  File(PathBuf),
  Stdout,
}

impl OutputTarget {
  pub(crate) fn resolve(&self, env: &Env) -> Self {
    match self {
      Self::File(path) => Self::File(env.resolve(path)),
      Self::Stdout => Self::Stdout,
    }
  }
}

impl From<&OsStr> for OutputTarget {
  fn from(text: &OsStr) -> Self {
    if text == OsStr::new("-") {
      Self::Stdout
    } else {
      Self::File(text.into())
    }
  }
}

impl Display for OutputTarget {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match self {
      Self::Stdout => write!(f, "standard output"),
      Self::File(path) => write!(f, "`{}`", path.display()),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn file() {
    assert_eq!(
      OutputTarget::from(OsStr::new("foo")),
      OutputTarget::File("foo".into())
    );
  }

  #[test]
  fn stdio() {
    assert_eq!(OutputTarget::from(OsStr::new("-")), OutputTarget::Stdout);
  }

  #[test]
  fn display_file() {
    let path = PathBuf::from("./path");
    let have = OutputTarget::File(path).to_string();
    let want = "`./path`";
    assert_eq!(have, want);
  }

  #[test]
  fn display_stdio() {
    let have = OutputTarget::Stdout.to_string();
    let want = "standard output";
    assert_eq!(have, want);
  }
}
