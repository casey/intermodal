use crate::common::*;

pub(crate) struct Env {
  args: Vec<OsString>,
  dir: PathBuf,
  input: Box<dyn Read>,
  err: OutputStream,
  out: OutputStream,
}

impl Env {
  pub(crate) fn main() -> Self {
    let dir = match env::current_dir() {
      Ok(dir) => dir,
      Err(error) => panic!("Failed to get current directory: {}", error),
    };

    let style = env::var_os("NO_COLOR").is_none()
      && env::var_os("TERM").as_deref() != Some(OsStr::new("dumb"));

    let out_stream = OutputStream::stdout(style);
    let err_stream = OutputStream::stderr(style);

    Self::new(
      dir,
      env::args(),
      Box::new(io::stdin()),
      out_stream,
      err_stream,
    )
  }

  pub(crate) fn run(&mut self) -> Result<(), Error> {
    #[cfg(windows)]
    ansi_term::enable_ansi_support().ok();

    Self::initialize_logging();

    let args = Arguments::from_iter_safe(&self.args)?;

    let use_color = args.options().use_color;
    self.err.set_use_color(use_color);
    self.out.set_use_color(use_color);

    args.run(self)
  }

  /// Initialize `pretty-env-logger` as the global logging backend.
  ///
  /// This function is called in `Env::run`, so the logger will always be
  /// initialized when the program runs via main, and in tests which construct
  /// and `Env` and run them.
  ///
  /// The logger will not be initialized in tests which don't construct an
  /// `Env`, for example in unit tests that test functionality below the level
  /// of a full program invocation.
  ///
  /// To enable logging in those tests, call `Env::initialize_logging()` like
  /// so:
  ///
  /// ```
  /// #[test]
  /// fn foo() {
  ///   Env::initialize_logging();
  ///   // Rest of the test...
  /// }
  /// ```
  ///
  /// If the logger has already been initialized, `Env::initialize_logging()` is
  /// a no-op, so it's safe to call more than once.
  pub(crate) fn initialize_logging() {
    static ONCE: Once = Once::new();

    ONCE.call_once(|| {
      pretty_env_logger::init();
    });
  }

  pub(crate) fn new<S, I>(
    dir: PathBuf,
    args: I,
    input: Box<dyn Read>,
    out: OutputStream,
    err: OutputStream,
  ) -> Self
  where
    S: Into<OsString>,
    I: IntoIterator<Item = S>,
  {
    Self {
      args: args.into_iter().map(Into::into).collect(),
      input,
      dir,
      out,
      err,
    }
  }

  pub(crate) fn status(&mut self) -> Result<(), i32> {
    use structopt::clap::ErrorKind;

    if let Err(error) = self.run() {
      if let Error::Clap { source } = error {
        if source.use_stderr() {
          write!(&mut self.err, "{}", source).ok();
        } else {
          write!(&mut self.out, "{}", source).ok();
        }
        match source.kind {
          ErrorKind::VersionDisplayed | ErrorKind::HelpDisplayed => Ok(()),
          _ => Err(EXIT_FAILURE),
        }
      } else {
        let style = self.err.style();
        writeln!(
          &mut self.err,
          "{}{}: {}{}",
          style.error().paint("error"),
          style.message().prefix(),
          error,
          style.message().suffix(),
        )
        .ok();

        if let Some(lint) = error.lint() {
          writeln!(
            &mut self.err,
            "{}: This check can be disabled with `--allow {}`.",
            style.message().paint("note"),
            lint.name()
          )
          .ok();
        }

        Err(EXIT_FAILURE)
      }
    } else {
      Ok(())
    }
  }

  pub(crate) fn dir(&self) -> &Path {
    &self.dir
  }

  pub(crate) fn err(&self) -> &OutputStream {
    &self.err
  }

  pub(crate) fn err_mut(&mut self) -> &mut OutputStream {
    &mut self.err
  }

  pub(crate) fn out(&self) -> &OutputStream {
    &self.out
  }

  pub(crate) fn out_mut(&mut self) -> &mut OutputStream {
    &mut self.out
  }

  pub(crate) fn resolve(&self, path: impl AsRef<Path>) -> PathBuf {
    self.dir().join(path).clean()
  }

  pub(crate) fn read(&mut self, source: InputTarget) -> Result<Input> {
    let data = match &source {
      InputTarget::File(path) => {
        let absolute = self.resolve(path);
        fs::read(absolute).context(error::Filesystem { path })?
      }
      InputTarget::Stdin => {
        let mut buffer = Vec::new();
        self.input.read_to_end(&mut buffer).context(error::Stdin)?;
        buffer
      }
    };

    Ok(Input::new(source, data))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn error_message_on_stdout() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "udp:bar.com",
        "--announce-tier",
        "foo",
      ],
      tree: {
        foo: "",
      }
    };
    env.status().ok();
    let err = env.err();
    if !err.starts_with("error: Failed to parse announce URL:") {
      panic!("Unexpected standard error output: {}", err);
    }

    assert_eq!(env.out(), "");
  }
}
