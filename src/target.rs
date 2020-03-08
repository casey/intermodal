use crate::common::*;

#[derive(PartialEq, Debug)]
pub(crate) enum Target {
  File(PathBuf),
  Stdio,
}

impl Target {
  pub(crate) fn resolve(&self, env: &Env) -> Self {
    match self {
      Self::File(path) => Self::File(env.resolve(path)),
      Self::Stdio => Self::Stdio,
    }
  }
}

impl From<&OsStr> for Target {
  fn from(text: &OsStr) -> Self {
    if text == OsStr::new("-") {
      Self::Stdio
    } else {
      Self::File(text.into())
    }
  }
}

impl Display for Target {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Stdio => write!(f, "standard I/O"),
            Self::File(path) =>write!(f, "`{}`", path.display()),
        }
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn file() {
    assert_eq!(Target::from(OsStr::new("foo")), Target::File("foo".into()));
  }

  #[test]
  fn stdio() {
    assert_eq!(Target::from(OsStr::new("-")), Target::Stdio);
  }

  #[test]
  fn display_file() {
      let path = PathBuf::from("./path");
      let have = Target::File(path).to_string();
      let want = "`./path`";
      assert_eq!(have, want);
  }

  #[test]
  fn display_stdio() {
      let have = Target::Stdio.to_string();
      let want = "standard I/O";
      assert_eq!(have, want);
  }
}
