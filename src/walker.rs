use crate::common::*;

pub(crate) struct Walker {
  include_junk: bool,
  include_hidden: bool,
  root: PathBuf,
}

impl Walker {
  pub(crate) fn new(root: &Path) -> Walker {
    Walker {
      include_junk: false,
      include_hidden: false,
      root: root.to_owned(),
    }
  }

  pub(crate) fn _include_junk(self) -> Self {
    Walker {
      include_junk: true,
      ..self
    }
  }

  pub(crate) fn _include_hidden(self) -> Self {
    Walker {
      include_hidden: true,
      ..self
    }
  }

  pub(crate) fn files(self) -> Result<Files, Error> {
    let mut paths = Vec::new();
    let mut total_size = 0;

    let junk: &[&OsStr] = &[OsStr::new("Thumbs.db"), OsStr::new("Desktop.ini")];

    for result in WalkDir::new(&self.root).sort_by(|a, b| a.file_name().cmp(b.file_name())) {
      let entry = result?;

      let path = entry.path();

      let file_name = entry.file_name();

      let metadata = entry.metadata()?;

      if !metadata.is_file() {
        continue;
      }

      if !self.include_hidden && file_name.to_string_lossy().starts_with('.') {
        continue;
      }

      if !self.include_hidden && Platform::hidden(path)? {
        continue;
      }

      if !self.include_junk && junk.contains(&file_name) {
        continue;
      }

      total_size += metadata.len();
      paths.push(entry.path().to_owned());
    }

    Ok(Files::new(self.root, Bytes::from(total_size)))
  }
}
