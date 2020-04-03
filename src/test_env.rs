use crate::common::*;

macro_rules! test_env {
  {
    args: [$($arg:expr),* $(,)?],
    $(cwd: $cwd:expr,)?
    $(input: $input:expr,)?
    $(err_style: $err_style:expr,)?
    tree: {
      $($tree:tt)*
    } $(,)?
  } => {
    {
      let tempdir = temptree! { $($tree)* };

      TestEnvBuilder::new()
        $(.current_dir(tempdir.path().join($cwd)))?
        $(.err_style($err_style))?
        $(.input($input))?
        .tempdir(tempdir)
        .arg("imdl")
        $(.arg($arg))*
        .build()
    }
  }
}

pub(crate) struct TestEnv {
  env: Env,
  #[allow(unused)]
  tempdir: TempDir,
  err: Capture,
  out: Capture,
}

impl TestEnv {
  pub(crate) fn new(tempdir: TempDir, env: Env, err: Capture, out: Capture) -> TestEnv {
    Self {
      tempdir,
      err,
      env,
      out,
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
    fs::write(self.env.resolve(path), bytes.as_ref()).unwrap();
  }

  pub(crate) fn remove_file(&self, path: impl AsRef<Path>) {
    fs::remove_file(self.env.resolve(path)).unwrap();
  }

  pub(crate) fn create_dir(&self, path: impl AsRef<Path>) {
    fs::create_dir(self.env.resolve(path)).unwrap();
  }

  #[cfg(unix)]
  pub(crate) fn metadata(&self, path: impl AsRef<Path>) -> fs::Metadata {
    fs::metadata(self.env.resolve(path)).unwrap()
  }

  #[cfg(unix)]
  pub(crate) fn set_permissions(&self, path: impl AsRef<Path>, permissions: fs::Permissions) {
    fs::set_permissions(self.env.resolve(path), permissions).unwrap();
  }

  pub(crate) fn assert_ok(&mut self) {
    match self.run() {
      Ok(()) => {}
      Err(err) => {
        eprintln!("Run failed: {}", err);
        eprintln!("Std error:\n{}", self.err());
        eprintln!("Std output:\n{}", self.out());
        panic!();
      }
    }
  }

  pub(crate) fn load_metainfo(&mut self, filename: impl AsRef<Path>) -> Metainfo {
    let input = self.env.read(filename.as_ref().as_os_str().into()).unwrap();
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
