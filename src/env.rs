use crate::common::*;

pub(crate) struct Env {
  args: Vec<String>,
  dir: Box<dyn AsRef<Path>>,
  pub(crate) err: Box<dyn Write>,
  pub(crate) _out: Box<dyn Write>,
}

impl Env {
  pub(crate) fn main() -> Self {
    let dir = match env::current_dir() {
      Ok(dir) => dir,
      Err(error) => panic!("Failed to get current directory: {}", error),
    };

    Self::new(dir, io::stdout(), io::stderr(), env::args())
  }

  pub(crate) fn run(&mut self) -> Result<(), Error> {
    Opt::from_iter_safe(&self.args)?.run(self)
  }

  pub(crate) fn new<D, O, E, S, I>(dir: D, out: O, err: E, args: I) -> Self
  where
    D: AsRef<Path> + 'static,
    O: Write + 'static,
    E: Write + 'static,
    S: Into<String>,
    I: IntoIterator<Item = S>,
  {
    Self {
      args: args.into_iter().map(Into::into).collect(),
      dir: Box::new(dir),
      err: Box::new(err),
      _out: Box::new(out),
    }
  }

  pub(crate) fn status(&mut self) -> Result<(), i32> {
    use structopt::clap::ErrorKind;
    if let Err(error) = self.run() {
      if let Error::Clap { source } = error {
        write!(&mut self.err, "{}", source).ok();
        match source.kind {
          ErrorKind::VersionDisplayed | ErrorKind::HelpDisplayed => Ok(()),
          _ => Err(EXIT_FAILURE),
        }
      } else {
        write!(&mut self.err, "error: {}", error).ok();
        Err(EXIT_FAILURE)
      }
    } else {
      Ok(())
    }
  }

  pub(crate) fn resolve(&self, path: impl AsRef<Path>) -> PathBuf {
    self.dir.as_ref().as_ref().join(path).clean()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn error_message_on_stdout() {
    let mut env = testing::env(
      ["torrent", "create", "--input", "foo", "--announce", "bar"]
        .iter()
        .cloned(),
    );
    fs::write(env.resolve("foo"), "").unwrap();
    env.status().ok();
    let err = env.err();
    if !err.starts_with("error: Failed to parse announce URL:") {
      panic!("Unexpected standard error output: {}", err);
    }

    assert_eq!(env.out(), "");
  }
}
