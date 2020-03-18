use crate::common::*;

#[derive(PartialEq, Debug, Clone)]
pub(crate) enum InputTarget {
  File(PathBuf),
  Stdin,
}

impl InputTarget {
  // TODO: remove
  pub(crate) fn resolve(&self, env: &Env) -> Self {
    match self {
      Self::File(path) => Self::File(env.resolve(path)),
      Self::Stdin => Self::Stdin,
    }
  }

  // TODO: remove
  pub(crate) fn read(&self) -> Result<Vec<u8>> {
    match self {
      Self::File(path) => fs::read(path).context(error::Filesystem { path }),
      Self::Stdin => {
        let mut buffer = Vec::new();
        io::stdin().read_to_end(&mut buffer).context(error::Stdin)?;
        Ok(buffer)
      }
    }
  }
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
