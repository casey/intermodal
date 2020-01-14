use crate::common::*;

pub(crate) struct TestEnv {
  env: Env,
  err: Capture,
  out: Capture,
}

impl TestEnv {
  pub(crate) fn new(iter: impl IntoIterator<Item = impl Into<String>>) -> Self {
    let err = Capture::new();
    let out = Capture::new();

    let env = Env::new(
      tempfile::tempdir().unwrap(),
      out.clone(),
      err.clone(),
      iter::once(String::from("imdl")).chain(iter.into_iter().map(|item| item.into())),
    );

    Self { err, env, out }
  }

  pub(crate) fn err(&self) -> String {
    self.err.string()
  }

  pub(crate) fn out(&self) -> String {
    self.out.string()
  }
}

impl Deref for TestEnv {
  type Target = Env;

  fn deref(&self) -> &Self::Target {
    &self.env
  }
}

impl DerefMut for TestEnv {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.env
  }
}
