use crate::common::*;

pub(crate) struct TestEnv {
  env: Env,
  err: Capture,
  out: Capture,
}

impl TestEnv {
  pub(crate) fn new(env: Env, err: Capture, out: Capture) -> TestEnv {
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
