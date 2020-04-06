use crate::common::*;

#[derive(PartialEq, Debug, Clone)]
pub(crate) enum OutputTarget {
  Path(PathBuf),
  Stdout,
}

impl OutputTarget {
  pub(crate) fn resolve(&self, env: &Env) -> Result<Self> {
    match self {
      Self::Path(path) => Ok(Self::Path(env.resolve(path)?)),
      Self::Stdout => Ok(Self::Stdout),
    }
  }

  pub(crate) fn try_from_os_str(text: &OsStr) -> Result<Self, OsString> {
    text
      .try_into()
      .map_err(|err: Error| OsString::from(err.to_string()))
  }
}

impl TryFrom<&OsStr> for OutputTarget {
  type Error = Error;

  fn try_from(text: &OsStr) -> Result<Self, Self::Error> {
    if text.is_empty() {
      return Err(Error::OutputTargetEmpty);
    };

    if text == OsStr::new("-") {
      Ok(Self::Stdout)
    } else {
      Ok(Self::Path(text.into()))
    }
  }
}

impl Display for OutputTarget {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match self {
      Self::Stdout => write!(f, "standard output"),
      Self::Path(path) => write!(f, "`{}`", path.display()),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn file() {
    assert_eq!(
      OutputTarget::try_from(OsStr::new("foo")).unwrap(),
      OutputTarget::Path("foo".into())
    );
  }

  #[test]
  fn stdio() {
    assert_eq!(
      OutputTarget::try_from(OsStr::new("-")).unwrap(),
      OutputTarget::Stdout
    );
  }

  #[test]
  fn display_file() {
    let path = PathBuf::from("./path");
    let have = OutputTarget::Path(path).to_string();
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
