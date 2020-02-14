use crate::common::*;

#[derive(Debug)]
pub(crate) struct FileStatus {
  path: PathBuf,
  error: Option<io::Error>,
  present: bool,
  file: bool,
  length_expected: Bytes,
  length_actual: Option<Bytes>,
  md5_expected: Option<Md5Digest>,
  md5_actual: Option<Md5Digest>,
}

impl FileStatus {
  pub(crate) fn status(
    path: &Path,
    length_expected: Bytes,
    md5_expected: Option<Md5Digest>,
  ) -> Self {
    let mut status = Self::new(path.to_owned(), length_expected, md5_expected);

    if let Err(error) = status.verify() {
      status.error = Some(error);
    }

    status
  }

  fn new(path: PathBuf, length_expected: Bytes, md5_expected: Option<Md5Digest>) -> Self {
    Self {
      error: None,
      file: false,
      md5_actual: None,
      present: false,
      length_actual: None,
      length_expected,
      md5_expected,
      path,
    }
  }

  fn verify(&mut self) -> io::Result<()> {
    let metadata = self.path.metadata()?;

    self.present = true;

    if !metadata.is_file() {
      return Ok(());
    }

    self.file = true;

    self.length_actual = Some(metadata.len().into());

    if self.md5_expected.is_some() {
      let mut reader = File::open(&self.path)?;
      let mut context = md5::Context::new();
      io::copy(&mut reader, &mut context)?;
      self.md5_actual = Some(context.compute().into());
    }

    Ok(())
  }

  pub(crate) fn icon(&self) -> char {
    if self.error.is_some() {
      return '!';
    }

    if !self.present {
      return '?';
    }

    if !self.file {
      return '¿';
    }

    if !self.md5() {
      return 'x';
    }

    let length = self.length_actual.unwrap();

    if length > self.length_expected {
      return '+';
    }

    if length < self.length_expected {
      return '-';
    }

    '♡'
  }

  fn md5(&self) -> bool {
    match (self.md5_actual, self.md5_expected) {
      (Some(actual), Some(expected)) => actual == expected,
      (None, None) => true,
      _ => unreachable!(),
    }
  }

  pub(crate) fn good(&self) -> bool {
    self.error.is_none() && self.present && self.file && self.md5()
  }

  pub(crate) fn bad(&self) -> bool {
    !self.good()
  }
  pub(crate) fn path(&self) -> &Path {
    &self.path
  }
}
