use crate::common::*;

pub(crate) struct Files {
  total_size: Bytes,
}

impl Files {
  pub(crate) fn from_root(root: &Path) -> Result<Files, Error> {
    let mut total_size = 0;

    for result in WalkDir::new(root).sort_by(|a, b| a.file_name().cmp(b.file_name())) {
      let entry = result?;

      let metadata = entry.metadata()?;

      if metadata.is_file() {
        total_size += metadata.len();
      }
    }

    Ok(Files {
      total_size: Bytes::from(total_size),
    })
  }

  pub(crate) fn total_size(&self) -> Bytes {
    self.total_size
  }
}
