use crate::common::*;

const JUNK: &[&str] = &["Thumbs.db", "Desktop.ini"];

pub(crate) struct Walker {
  follow_symlinks: bool,
  include_hidden: bool,
  include_junk: bool,
  root: PathBuf,
}

impl Walker {
  pub(crate) fn new(root: &Path) -> Walker {
    Walker {
      follow_symlinks: false,
      include_hidden: false,
      include_junk: false,
      root: root.to_owned(),
    }
  }

  pub(crate) fn include_junk(self, include_junk: bool) -> Self {
    Walker {
      include_junk,
      ..self
    }
  }

  pub(crate) fn include_hidden(self, include_hidden: bool) -> Self {
    Walker {
      include_hidden,
      ..self
    }
  }

  pub(crate) fn follow_symlinks(self, follow_symlinks: bool) -> Self {
    Walker {
      follow_symlinks,
      ..self
    }
  }

  pub(crate) fn files(self) -> Result<Files, Error> {
    if !self.follow_symlinks
      && self
        .root
        .symlink_metadata()
        .context(error::Filesystem { path: &self.root })?
        .file_type()
        .is_symlink()
    {
      return Err(Error::SymlinkRoot { root: self.root });
    }

    let root_metadata = self
      .root
      .metadata()
      .context(error::Filesystem { path: &self.root })?;

    if root_metadata.is_file() {
      return Ok(Files::file(self.root, Bytes::from(root_metadata.len())));
    }

    let filter = |entry: &walkdir::DirEntry| {
      let path = entry.path();

      let file_name = entry.file_name();

      if !self.include_hidden && file_name.to_string_lossy().starts_with('.') {
        return false;
      }

      let hidden = Platform::hidden(path).unwrap_or(true);

      if !self.include_hidden && hidden {
        return false;
      }

      true
    };

    let mut paths = Vec::new();
    let mut total_size = 0;
    for result in WalkDir::new(&self.root)
      .follow_links(self.follow_symlinks)
      .sort_by(|a, b| a.file_name().cmp(b.file_name()))
      .into_iter()
      .filter_entry(filter)
    {
      let entry = result?;

      let path = entry.path();

      let metadata = entry.metadata()?;

      if !metadata.is_file() {
        continue;
      }

      let file_path = FilePath::from_prefix_and_path(&self.root, &path)?;

      if !self.include_junk && JUNK.contains(&file_path.name()) {
        continue;
      }

      total_size += metadata.len();

      paths.push(file_path);
    }

    Ok(Files::dir(self.root, Bytes::from(total_size), paths))
  }
}
