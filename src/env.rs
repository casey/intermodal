use crate::common::*;

pub(crate) struct Env {
  args: Vec<OsString>,
  dir: PathBuf,
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

    Self::new(dir, env::args(), out_stream, err_stream)
  }

  pub(crate) fn run(&mut self) -> Result<(), Error> {
    #[cfg(windows)]
    ansi_term::enable_ansi_support().ok();

    #[cfg(not(test))]
    pretty_env_logger::init();

    let args = Arguments::from_iter_safe(&self.args)?;

    let use_color = args.options().use_color;
    self.err.set_use_color(use_color);
    self.out.set_use_color(use_color);

    args.run(self)
  }

  pub(crate) fn new<S, I>(dir: PathBuf, args: I, out: OutputStream, err: OutputStream) -> Self
  where
    S: Into<OsString>,
    I: IntoIterator<Item = S>,
  {
    Self {
      args: args.into_iter().map(Into::into).collect(),
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

        error.print_body(self).ok();

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

  pub(crate) fn resolve(&self, path: impl AsRef<Path>) -> PathBuf {
    self.dir().join(path).clean()
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
