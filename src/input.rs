use crate::common::*;

pub(crate) struct Input {
  pub(crate) source: InputTarget,
  pub(crate) data: Vec<u8>,
}

impl Input {
  #[cfg(test)]
  pub(crate) fn from_path(path: &Path) -> Result<Input> {
    let data = fs::read(path).context(error::Filesystem { path })?;
    Ok(Input {
      source: InputTarget::Path(path.to_owned()),
      data,
    })
  }
}
