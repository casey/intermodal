use crate::common::*;

#[serde(transparent)]
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
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

    assert!(!components.is_empty());

    Ok(FilePath { components })
  }

  pub(crate) fn name(&self) -> &str {
    &self.components[0]
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
    let components: Vec<String> = components
      .iter()
      .cloned()
      .map(|component| component.to_owned())
      .collect();
    assert!(!components.is_empty());
    FilePath { components }
  }
}
