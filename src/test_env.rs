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

  pub(crate) fn out_bytes(&self) -> Vec<u8> {
    self.out.bytes()
  }

  pub(crate) fn create_dir(&self, path: impl AsRef<Path>) {
    fs::create_dir_all(self.env.resolve(path.as_ref())).unwrap();
  }

  pub(crate) fn create_file(&self, path: impl AsRef<Path>, bytes: impl AsRef<[u8]>) {
    fs::write(self.env.resolve(path), bytes.as_ref()).unwrap();
  }

  pub(crate) fn load_metainfo(&self, filename: impl AsRef<Path>) -> Metainfo {
    Metainfo::load(self.env.resolve(filename.as_ref())).unwrap()
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
