use crate::common::*;

pub(crate) struct Input {
  source: InputTarget,
  data: Vec<u8>,
}

impl Input {
  pub(crate) fn new(source: InputTarget, data: Vec<u8>) -> Input {
    Self { source, data }
  }

  pub(crate) fn data(&self) -> &[u8] {
    &self.data
  }

  pub(crate) fn source(&self) -> &InputTarget {
    &self.source
  }

  #[cfg(test)]
  pub(crate) fn from_path(path: &Path) -> Result<Input> {
    let data = fs::read(path).context(error::Filesystem { path })?;
    Ok(Input {
      source: InputTarget::Path(path.to_owned()),
      data,
    })
  }
}
