use crate::common::*;

pub(crate) struct TestEnvBuilder {
  args: Vec<OsString>,
  current_dir: Option<PathBuf>,
  out_is_term: bool,
  tempdir: Option<TempDir>,
  use_color: bool,
}

impl TestEnvBuilder {
  pub(crate) fn new() -> TestEnvBuilder {
    TestEnvBuilder {
      args: Vec::new(),
      current_dir: None,
      out_is_term: false,
      tempdir: None,
      use_color: false,
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

  pub(crate) fn current_dir(mut self, path: PathBuf) -> Self {
    self.current_dir = Some(path);
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

    let tempdir = self.tempdir.unwrap_or_else(|| tempfile::tempdir().unwrap());

    let current_dir = if let Some(current_dir) = self.current_dir {
      tempdir.path().join(current_dir)
    } else {
      tempdir.path().to_owned()
    };

    let env = Env::new(
      current_dir,
      out.clone(),
      if self.use_color && self.out_is_term {
        Style::active()
      } else {
        Style::inactive()
      },
      self.out_is_term,
      err.clone(),
      Style::inactive(),
      false,
      self.args,
    );

    TestEnv::new(tempdir, env, err, out)
  }
}
