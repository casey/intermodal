use crate::common::*;

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, Ord, PartialOrd, Eq)]
#[serde(transparent)]
pub(crate) struct FilePath {
  components: Vec<String>,
}

impl FilePath {
  pub(crate) fn from_relative_path(path: &Path) -> Result<FilePath, Error> {
    let mut components = Vec::new();

    for component in path.components() {
      match component {
        path::Component::Normal(os) => {
          if let Some(unicode) = os.to_str() {
            components.push(unicode.to_owned());
          } else {
            return Err(Error::PathDecode {
              path: path.to_owned(),
              component: PathBuf::from(component.as_os_str()),
            });
          }
        }
        _ => {
          return Err(Error::PathComponent {
            path: path.to_owned(),
            component: PathBuf::from(component.as_os_str()),
          })
        }
      }
    }

    if components.is_empty() {
      return Err(Error::internal("FilePath::from_relative_path: empty path"));
    }

    Ok(FilePath { components })
  }

  pub(crate) fn name(&self) -> &str {
    &self.components[self.components.len() - 1]
  }

  pub(crate) fn components(&self) -> &[String] {
    &self.components
  }

  pub(crate) fn absolute(&self, root: &Path) -> PathBuf {
    let mut absolute = root.to_owned();
    for component in &self.components {
      absolute.push(component);
    }
    absolute
  }

  #[cfg(test)]
  pub(crate) fn from_components(components: &[&str]) -> FilePath {
    let components: Vec<String> = components.iter().copied().map(ToOwned::to_owned).collect();
    assert!(!components.is_empty());
    FilePath { components }
  }
}

impl Display for FilePath {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    for (i, component) in self.components.iter().enumerate() {
      if i > 0 {
        write!(f, "/")?;
      }
      write!(f, "{component}")?;
    }
    Ok(())
  }
}
