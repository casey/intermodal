use crate::common::*;

use std::path::Component;

pub(crate) trait PathExt {
  fn clean(self) -> PathBuf;
}

impl PathExt for &Path {
  fn clean(self) -> PathBuf {
    if self.components().count() <= 1 {
      return self.to_owned();
    }

    let mut components = Vec::new();

    for component in self
      .components()
      .filter(|component| component != &Component::CurDir)
    {
      if component == Component::ParentDir {
        match components.last() {
          Some(Component::Normal(_)) => {
            components.pop();
          }
          Some(Component::ParentDir) | None => components.push(component),
          _ => {}
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
  #[rustfmt::skip]
  fn prefix_suffix() {
    fn case(prefix: &str, suffix: &str, want: &str) {
      let have = Path::new(prefix).join(Path::new(suffix)).clean();
      assert_eq!(have, Path::new(want));
    }

    {
      case("/",            "foo",       "/foo");
      case("/",            "." ,        "/");
      case("/",            "foo/./bar", "/foo/bar");
      case("/foo/./bar",   ".",         "/foo/bar");
      case("/bar",         "/foo",      "/foo");
      case("//foo",        "bar//baz",  "/foo/bar/baz");
      case("/",            "..",        "/");
      case("/",            "/..",       "/");
      case("/..",          "",          "/");
      case("/../../../..", "../../../", "/");
      case("/.",           "./",        "/");
      case("/foo/../",     "bar",       "/bar");
      case("/foo/bar",     "..",        "/foo");
      case("/foo/bar/",    "..",        "/foo");
    }
  }

  #[test]
  #[rustfmt::skip]
  fn simple() {
    fn case(path: &str, want: &str) {
      assert_eq!(Path::new(path).clean(), Path::new(want));
    }

    case("./..",      "..");
    case("./././.",   ".");
    case("./../.",    "..");
    case("..",        "..");
    case("",          "");
    case("foo",       "foo");
    case(".",         ".");
    case("foo/./bar", "foo/bar");
    case("/foo",      "/foo");
    case("bar//baz",  "bar/baz");
    case("/..",       "/");
    case("../../../", "../../..");
    case("./",        ".");
  }
}
