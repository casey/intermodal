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
        return Err(if error.kind() == io::ErrorKind::NotFound {
          FileError::Missing
        } else {
          FileError::Io(error)
        })
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

impl Print for FileError {
  fn print(&self, stream: &mut OutputStream) -> io::Result<()> {
    let style = stream.style();

    if let Self::Md5 { actual, expected } = self {
      write!(
        stream,
        "MD5 checksum mismatch: {} (expected {})",
        style.error().paint(actual.to_string()),
        style.good().paint(expected.to_string()),
      )?;

      return Ok(());
    }

    match self {
      Self::Io(io_error) => write!(stream, "{io_error}")?,
      Self::Missing => write!(stream, "File missing")?,
      Self::Directory => write!(stream, "Expected file but found directory")?,
      Self::Surfeit(difference) => write!(stream, "{difference} too long")?,
      Self::Dearth(difference) => write!(stream, "{difference} too short")?,
      Self::Md5 { .. } => {
        return Err(io::Error::new(
          io::ErrorKind::Other,
          Error::internal("Reached unreachable branch").to_string(),
        ))
      }
    }

    Ok(())
  }
}
