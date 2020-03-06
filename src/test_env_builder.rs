use crate::common::*;

pub(crate) struct TestEnvBuilder {
  args: Vec<OsString>,
  out_is_term: bool,
  use_color: bool,
  tempdir: Option<TempDir>,
}

impl TestEnvBuilder {
  pub(crate) fn new() -> TestEnvBuilder {
    TestEnvBuilder {
      args: Vec::new(),
      out_is_term: false,
      use_color: false,
      tempdir: None,
    }
  }

  pub(crate) fn out_is_term(mut self) -> Self {
    self.out_is_term = true;
    self
  }

  pub(crate) fn arg(mut self, arg: impl Into<OsString>) -> Self {
    self.args.push(arg.into());
    self
  }

  pub(crate) fn arg_slice(mut self, args: &[&str]) -> Self {
    for arg in args.iter().cloned() {
      self.args.push(arg.into());
    }
    self
  }

  pub(crate) fn tempdir(mut self, tempdir: TempDir) -> Self {
    self.tempdir = Some(tempdir);
    self
  }

  pub(crate) fn build(self) -> TestEnv {
    let err = Capture::new();
    let out = Capture::new();

    let env = Env::new(
      self.tempdir.unwrap_or_else(|| tempfile::tempdir().unwrap()),
      out.clone(),
      if self.use_color && self.out_is_term {
        Style::active()
      } else {
        Style::inactive()
      },
      self.out_is_term,
      err.clone(),
      Style::inactive(),
      self.args,
    );

    TestEnv::new(env, err, out)
  }
}
