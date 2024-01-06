use crate::common::*;

pub(crate) struct Env {
  args: Vec<OsString>,
  dir: PathBuf,
  input: Box<dyn InputStream>,
  err: OutputStream,
  out: OutputStream,
}

impl Env {
  pub(crate) fn main() -> Result<Self> {
    let dir = env::current_dir().context(error::CurrentDirectoryGet)?;

    let style = env::var_os("NO_COLOR").is_none()
      && env::var_os("TERM").as_deref() != Some(OsStr::new("dumb"));

    let out_stream = OutputStream::stdout(style);
    let err_stream = OutputStream::stderr(style);

    Ok(Self::new(
      dir,
      env::args(),
      Box::new(io::stdin()),
      out_stream,
      err_stream,
    ))
  }

  pub(crate) fn run(&mut self) -> Result<()> {
    #[cfg(windows)]
    ansi_term::enable_ansi_support().ok();

    Self::initialize_logging();

    let app = {
      let mut app = Arguments::clap();

      let width = env::var("IMDL_TERM_WIDTH")
        .ok()
        .and_then(|width| width.parse::<usize>().ok());

      if let Some(width) = width {
        app = app.set_term_width(width);
      }

      app
    };

    let matches = app.get_matches_from_safe(&self.args)?;

    let args = Arguments::from_clap(&matches);

    let use_color = args.options().use_color;
    self.err.set_use_color(use_color);
    self.out.set_use_color(use_color);

    if args.options().terminal {
      self.err.set_is_term(true);
      self.out.set_is_term(true);
    }

    if args.options().quiet {
      self.err.set_active(false);
    }

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
  /// ```no_run
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
    input: Box<dyn InputStream>,
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
          write!(&mut self.err, "{source}").ok();
        } else {
          write!(&mut self.out, "{source}").ok();
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

  pub(crate) fn input<'a>(&'a mut self) -> Box<dyn BufRead + 'a> {
    self.input.as_mut().buf_read()
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

  pub(crate) fn resolve(&self, path: impl AsRef<Path>) -> Result<PathBuf> {
    let path = path.as_ref();

    if path.components().count() == 0 {
      return Err(Error::internal("Empty path passed to resolve"));
    }

    Ok(self.dir().join(path).lexiclean())
  }

  pub(crate) fn write(&mut self, path: impl AsRef<Path>, contents: impl AsRef<[u8]>) -> Result<()> {
    let path = path.as_ref();
    fs::write(self.resolve(path)?, contents).context(error::Filesystem { path })
  }

  pub(crate) fn read(&mut self, source: InputTarget) -> Result<Input> {
    let data = match &source {
      InputTarget::Path(path) => {
        let absolute = self.resolve(path)?;
        fs::read(absolute).context(error::Filesystem { path })?
      }
      InputTarget::Stdin => {
        let mut buffer = Vec::new();
        self
          .input
          .buf_read()
          .read_to_end(&mut buffer)
          .context(error::Stdin)?;
        buffer
      }
    };

    Ok(Input { source, data })
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
    assert!(
      err.starts_with("error: Failed to parse announce URL:"),
      "Unexpected standard error output: {err}",
    );

    assert_eq!(env.out(), "");
  }

  #[test]
  fn quiet() {
    let mut env = test_env! {
      args: [
        "--quiet",
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
    assert_eq!(env.err(), "");
    assert_eq!(env.out(), "");
  }

  #[test]
  fn terminal() -> Result<()> {
    let mut create_env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "udp:bar.com",
      ],
      tree: {
        foo: "",
      }
    };

    create_env.assert_ok();

    let mut env = test_env! {
      args: [
        "--terminal",
        "torrent",
        "show",
        "--input",
        create_env.resolve("foo.torrent")?,
      ],
      tree: {
      }
    };

    env.assert_ok();

    assert_eq!(env.err(), "");
    assert!(env.out().starts_with("         Name"));

    Ok(())
  }
}
