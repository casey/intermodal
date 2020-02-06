use crate::common::*;

#[derive(Debug)]
pub(crate) struct Files {
  root: PathBuf,
  total_size: Bytes,
  contents: Option<Vec<FilePath>>,
}

impl Files {
  pub(crate) fn file(root: PathBuf, total_size: Bytes) -> Files {
    Files {
      contents: None,
      root,
      total_size,
    }
  }

  pub(crate) fn dir(root: PathBuf, total_size: Bytes, contents: Vec<FilePath>) -> Files {
    Files {
      contents: Some(contents),
      root,
      total_size,
    }
  }

  pub(crate) fn root(&self) -> &Path {
    &self.root
  }

  pub(crate) fn contents(&self) -> Option<&[FilePath]> {
    self.contents.as_deref()
  }

  pub(crate) fn total_size(&self) -> Bytes {
    self.total_size
  }
}
