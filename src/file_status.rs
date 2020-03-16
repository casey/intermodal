use crate::common::*;

#[derive(Debug)]
pub(crate) struct FileStatus {
  path: FilePath,
  error: Option<FileError>,
}

impl FileStatus {
  pub(crate) fn status(
    absolute: &Path,
    path: FilePath,
    length: Bytes,
    md5: Option<Md5Digest>,
  ) -> Self {
    let error = FileError::verify(absolute, length, md5).err();

    FileStatus { path, error }
  }

  pub(crate) fn is_good(&self) -> bool {
    self.error.is_none()
  }

  #[cfg(test)]
  pub(crate) fn is_bad(&self) -> bool {
    !self.is_good()
  }

  pub(crate) fn error(&self) -> Option<&FileError> {
    self.error.as_ref()
  }

  pub(crate) fn path(&self) -> &FilePath {
    &self.path
  }
}
