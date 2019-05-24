use crate::common::*;

pub(crate) struct Environment {
  args: Vec<String>,
  dir: Box<dyn AsRef<Path>>,
  err: Box<dyn Write>,
}

impl Environment {
  pub(crate) fn main() -> Environment {
    let dir = match env::current_dir() {
      Ok(dir) => dir,
      Err(error) => panic!("Failed to get current directory: {}", error),
    };

    Environment::new(dir, io::stderr(), env::args())
  }

  pub(crate) fn run(&self) -> Result<(), Error> {
    Opt::from_iter_safe(&self.args)?.run(self)
  }

  pub(crate) fn new<D, E, S, I>(dir: D, err: E, args: I) -> Environment
  where
    D: AsRef<Path> + 'static,
    E: Write + 'static,
    S: Into<String>,
    I: IntoIterator<Item = S>,
  {
    Environment {
      args: args.into_iter().map(|s| s.into()).collect(),
      dir: Box::new(dir),
      err: Box::new(err),
    }
  }

  pub(crate) fn status(&mut self) -> Result<(), i32> {
    if let Err(error) = self.run() {
      write!(&mut self.err, "error: {}", error).ok();
      Err(EXIT_FAILURE)
    } else {
      Ok(())
    }
  }

  pub(crate) fn resolve(&self, path: impl AsRef<Path>) -> PathBuf {
    self.dir.as_ref().as_ref().join(path).clean()
  }
}
