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
}
