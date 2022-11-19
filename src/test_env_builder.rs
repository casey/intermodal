use crate::common::*;

pub(crate) struct TestEnvBuilder {
  args: Vec<OsString>,
  current_dir: Option<PathBuf>,
  err_style: bool,
  input: Option<Box<dyn InputStream>>,
  out_is_term: bool,
  tempdir: Option<TempDir>,
  use_color: bool,
}

impl TestEnvBuilder {
  pub(crate) fn new() -> TestEnvBuilder {
    TestEnvBuilder {
      args: Vec::new(),
      current_dir: None,
      err_style: false,
      input: None,
      out_is_term: false,
      tempdir: None,
      use_color: false,
    }
  }

  pub(crate) fn out_is_term(mut self) -> Self {
    self.out_is_term = true;
    self
  }

  pub(crate) fn err_style(mut self, err_style: bool) -> Self {
    self.err_style = err_style;
    self
  }

  pub(crate) fn input(mut self, input: impl AsRef<[u8]>) -> Self {
    self.input = Some(Box::new(io::Cursor::new(input.as_ref().to_owned())));
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
    for arg in args.iter().copied() {
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

    let current_dir = self.current_dir.map_or_else(
      || tempdir.path().to_owned(),
      |current_dir| tempdir.path().join(current_dir),
    );

    let out_stream = OutputStream::new(
      Box::new(out.clone()),
      self.use_color && self.out_is_term,
      self.out_is_term,
      true,
    );

    let err_stream = OutputStream::new(Box::new(err.clone()), self.err_style, false, true);

    let env = Env::new(
      current_dir,
      self.args,
      self.input.unwrap_or_else(|| Box::new(io::empty())),
      out_stream,
      err_stream,
    );

    TestEnv::new(tempdir, env, err, out)
  }
}
