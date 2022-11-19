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
  ignore: bool,
  sort_by: Vec<SortSpec>,
  patterns: Vec<Pattern>,
  root: PathBuf,
  spinner: Option<ProgressBar>,
}

impl Walker {
  pub(crate) fn new(root: &Path) -> Self {
    Self {
      follow_symlinks: false,
      include_hidden: false,
      include_junk: false,
      ignore: false,
      sort_by: Vec::new(),
      patterns: Vec::new(),
      root: root.to_owned(),
      spinner: None,
    }
  }

  pub(crate) fn include_junk(self, include_junk: bool) -> Self {
    Self {
      include_junk,
      ..self
    }
  }

  pub(crate) fn include_hidden(self, include_hidden: bool) -> Self {
    Self {
      include_hidden,
      ..self
    }
  }

  pub(crate) fn ignore(self, ignore: bool) -> Self {
    Self { ignore, ..self }
  }

  pub(crate) fn sort_by(self, sort_by: Vec<SortSpec>) -> Self {
    Self { sort_by, ..self }
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
    Self {
      follow_symlinks,
      ..self
    }
  }

  pub(crate) fn spinner(self, spinner: Option<ProgressBar>) -> Self {
    Self { spinner, ..self }
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

    let mut file_infos = Vec::new();
    let mut total_size = 0;

    let mut walk_builder = WalkBuilder::new(&self.root);
    walk_builder
      .follow_links(self.follow_symlinks)
      .standard_filters(self.ignore)
      .require_git(false)
      .hidden(!self.include_hidden);
    for result in walk_builder.build() {
      let entry = result?;
      let path = entry.path();

      if let Some(s) = &self.spinner {
        let display_path = path.strip_prefix(&self.root).unwrap_or(path);
        s.set_message(&display_path.display().to_string());
        s.tick();
      }

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

      if !self.pattern_filter(relative) {
        continue;
      }

      let file_path = FilePath::from_relative_path(relative)?;

      if !self.include_junk && JUNK.contains(&file_path.name()) {
        continue;
      }

      let len = metadata.len();
      total_size += len;

      file_infos.push(FileInfo {
        path: file_path,
        length: Bytes(len),
        md5sum: None,
      });
    }

    file_infos.sort_by(|a, b| SortSpec::compare(&self.sort_by, a, b));

    Ok(Files::dir(
      self.root,
      Bytes::from(total_size),
      file_infos
        .into_iter()
        .map(|file_info| file_info.path)
        .collect(),
    ))
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
