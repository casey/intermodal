use crate::common::*;

const JUNK: &[&str] = &["Thumbs.db", "Desktop.ini"];

#[derive(Debug)]
struct Pattern {
  glob: GlobMatcher,
  include: bool,
}

pub(crate) struct Walker {
  follow_symlinks: bool,
  include_hidden: bool,
  include_junk: bool,
  patterns: Vec<Pattern>,
  root: PathBuf,
  spinner: Option<ProgressBar>,
}

impl Walker {
  pub(crate) fn new(root: &Path) -> Walker {
    Walker {
      follow_symlinks: false,
      include_hidden: false,
      include_junk: false,
      patterns: Vec::new(),
      root: root.to_owned(),
      spinner: None,
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

  pub(crate) fn globs(mut self, globs: &[String]) -> Result<Self, Error> {
    for glob in globs {
      let exclude = glob.starts_with('!');
      let glob = Glob::new(if exclude { &glob[1..] } else { glob })?.compile_matcher();
      self.patterns.push(Pattern {
        glob,
        include: !exclude,
      });
    }

    Ok(self)
  }

  pub(crate) fn follow_symlinks(self, follow_symlinks: bool) -> Self {
    Walker {
      follow_symlinks,
      ..self
    }
  }

  pub(crate) fn spinner(self, spinner: ProgressBar) -> Self {
    Walker {
      spinner: Some(spinner),
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

      if let Some(s) = &self.spinner {
        let display_path = path.strip_prefix(&self.root).unwrap_or(&path);
        s.set_message(&display_path.display().to_string());
        s.tick();
      }

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

      let relative = path
        .strip_prefix(&self.root)
        .context(error::PathStripPrefix {
          path,
          prefix: &self.root,
        })?;

      if relative.components().count() == 0 {
        return Err(Error::PathStripEmpty {
          prefix: self.root.clone(),
          path: path.to_owned(),
        });
      }

      if !self.pattern_filter(&relative) {
        continue;
      }

      let file_path = FilePath::from_relative_path(relative)?;

      if !self.include_junk && JUNK.contains(&file_path.name()) {
        continue;
      }

      total_size += metadata.len();

      paths.push(file_path);
    }

    Ok(Files::dir(self.root, Bytes::from(total_size), paths))
  }

  fn pattern_filter(&self, relative: &Path) -> bool {
    for Pattern { glob, include } in self.patterns.iter().rev() {
      if glob.is_match(relative) {
        return *include;
      }
    }

    if let Some(Pattern { include, .. }) = self.patterns.first() {
      return !include;
    }

    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn glob() {
    let walker = Walker::new(Path::new("foo"))
      .globs(&["[bc]".into()])
      .unwrap();

    assert!(!walker.pattern_filter(Path::new("a")));
    assert!(walker.pattern_filter(Path::new("b")));
    assert!(walker.pattern_filter(Path::new("c")));
  }
}
