use crate::common::*;

pub(crate) struct Files {
  root: PathBuf,
  total_size: Bytes,
}

impl Files {
  pub(crate) fn new(root: PathBuf, total_size: Bytes) -> Files {
    Files { root, total_size }
  }

  pub(crate) fn root(&self) -> &Path {
    &self.root
  }

  pub(crate) fn total_size(&self) -> Bytes {
    self.total_size
  }
}
