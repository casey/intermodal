use crate::common::*;

#[derive(Debug)]
pub(crate) enum FileError {
  Io(io::Error),
  Missing,
  Directory,
  Surfeit(Bytes),
  Dearth(Bytes),
  Md5 {
    expected: Md5Digest,
    actual: Md5Digest,
  },
}

impl FileError {
  pub(crate) fn verify(
    path: &Path,
    expected_length: Bytes,
    expected_md5: Option<Md5Digest>,
  ) -> Result<(), FileError> {
    let metadata = match path.metadata() {
      Ok(metadata) => metadata,
      Err(error) => {
        if error.kind() == io::ErrorKind::NotFound {
          return Err(FileError::Missing);
        } else {
          return Err(FileError::Io(error));
        }
      }
    };

    if metadata.is_dir() {
      return Err(FileError::Directory);
    }

    let actual = Bytes(metadata.len());

    let difference = actual.absolute_difference(expected_length);

    if actual > expected_length {
      return Err(FileError::Surfeit(difference));
    }

    if actual < expected_length {
      return Err(FileError::Dearth(difference));
    }

    if let Some(expected) = expected_md5 {
      let mut reader = File::open(path)?;
      let mut context = md5::Context::new();
      io::copy(&mut reader, &mut context)?;
      let actual = context.compute().into();

      if actual != expected {
        return Err(FileError::Md5 { actual, expected });
      }
    }

    Ok(())
  }
}

impl From<io::Error> for FileError {
  fn from(io_error: io::Error) -> Self {
    Self::Io(io_error)
  }
}

impl Display for FileError {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match self {
      Self::Io(io_error) => write!(f, "{}", io_error),
      Self::Missing => write!(f, "File missing"),
      Self::Directory => write!(f, "Expected file but found directory"),
      Self::Surfeit(difference) => write!(f, "Extra bytes: {}", difference),
      Self::Dearth(difference) => write!(f, "Missing bytes: {}", difference),
      Self::Md5 { actual, expected } => write!(
        f,
        "MD5 checksum mismatch: {} (expected {})",
        actual, expected
      ),
    }
  }
}

impl Print for FileError {
  fn print(&self, stream: &mut OutputStream) -> io::Result<()> {
    let style = stream.style();
    write!(
      stream,
      "{}{}{}",
      style.error().prefix(),
      self,
      style.error().suffix(),
    )
  }
}
