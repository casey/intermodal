use crate::common::*;

macro_rules! test_env {
  {
    args: [$($arg:expr),* $(,)?],
    $(cwd: $cwd:expr,)?
    $(input: $input:expr,)?
    $(err_style: $err_style:expr,)?
    tree: {
      $($tree:tt)*
    }
    $(, matches: $result:pat)?

    $(,)?
  } => {
    {
      let tempdir = temptree! { $($tree)* };

      let env = TestEnvBuilder::new()
        $(.current_dir(tempdir.path().join($cwd)))?
        $(.err_style($err_style))?
        $(.input($input))?
        .tempdir(tempdir)
        .arg("imdl")
        $(.arg($arg))*
        .build();

      $(
        let mut env = env;
        assert_matches!(env.run(), $result);
      )?

      env
    }
  }
}

pub(crate) struct TestEnv {
  env: Env,
  err: Capture,
  out: Capture,
  #[allow(unused)]
  tempdir: TempDir,
}

impl TestEnv {
  pub(crate) fn new(tempdir: TempDir, env: Env, err: Capture, out: Capture) -> TestEnv {
    Self {
      env,
      err,
      out,
      tempdir,
    }
  }

  pub(crate) fn err(&self) -> String {
    self.err.string()
  }

  pub(crate) fn out(&self) -> String {
    self.out.string()
  }

  pub(crate) fn out_bytes(&self) -> Vec<u8> {
    self.out.bytes()
  }

  pub(crate) fn write(&self, path: impl AsRef<Path>, bytes: impl AsRef<[u8]>) {
    fs::write(self.env.resolve(path).unwrap(), bytes.as_ref()).unwrap();
  }

  pub(crate) fn remove_file(&self, path: impl AsRef<Path>) {
    fs::remove_file(self.env.resolve(path).unwrap()).unwrap();
  }

  pub(crate) fn create_dir(&self, path: impl AsRef<Path>) {
    fs::create_dir(self.env.resolve(path).unwrap()).unwrap();
  }

  pub(crate) fn rename(&self, from: impl AsRef<Path>, to: impl AsRef<Path>) {
    fs::rename(
      self.env.resolve(from).unwrap(),
      self.env.resolve(to).unwrap(),
    )
    .unwrap();
  }

  pub(crate) fn read_to_string(&self, path: impl AsRef<Path>) -> String {
    fs::read_to_string(self.env.resolve(path).unwrap()).unwrap()
  }

  #[cfg(unix)]
  pub(crate) fn metadata(&self, path: impl AsRef<Path>) -> fs::Metadata {
    fs::metadata(self.env.resolve(path).unwrap()).unwrap()
  }

  #[cfg(unix)]
  pub(crate) fn set_permissions(&self, path: impl AsRef<Path>, permissions: fs::Permissions) {
    fs::set_permissions(self.env.resolve(path).unwrap(), permissions).unwrap();
  }

  pub(crate) fn assert_ok(&mut self) {
    match self.run() {
      Ok(()) => {}
      Err(err) => {
        eprintln!("Run failed: {err}");
        eprintln!("Std error:\n{}", self.err());
        eprintln!("Std output:\n{}", self.out());
        panic!();
      }
    }
  }

  pub(crate) fn load_metainfo(&mut self, filename: impl AsRef<Path>) -> Metainfo {
    let path = filename.as_ref();
    let target = InputTarget::try_from(path.as_os_str()).unwrap();
    let input = self.env.read(target).unwrap();
    Metainfo::from_input(&input).unwrap()
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
