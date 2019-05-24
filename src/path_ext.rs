use crate::common::*;

use std::path::Component;

pub(crate) trait PathExt {
  fn clean(self) -> PathBuf;
}

impl PathExt for &Path {
  fn clean(self) -> PathBuf {
    let mut components = Vec::new();

    for component in self.components() {
      if component == Component::ParentDir {
        if let Some(Component::Normal(_)) = components.last() {
          components.pop();
        }
      } else {
        components.push(component);
      }
    }

    components.into_iter().collect()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn clean() {
    let cases = &[
      ("/", "foo", "/foo"),
      ("/", ".", "/"),
      ("/", "foo/./bar", "/foo/bar"),
      ("/foo/./bar", ".", "/foo/bar"),
      ("/bar", "/foo", "/foo"),
      ("//foo", "bar//baz", "/foo/bar/baz"),
      ("/", "..", "/"),
      ("/", "/..", "/"),
      ("/..", "", "/"),
      ("/../../../..", "../../../", "/"),
      ("/.", "./", "/"),
      ("/foo/../", "bar", "/bar"),
      ("/foo/bar", "..", "/foo"),
      ("/foo/bar/", "..", "/foo"),
    ];

    for (prefix, suffix, want) in cases {
      let have = Path::new(prefix).join(Path::new(suffix)).clean();
      assert_eq!(have, Path::new(want));
    }
  }
}
