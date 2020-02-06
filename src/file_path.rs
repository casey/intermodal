use crate::common::*;

#[serde(transparent)]
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub(crate) struct FilePath {
  components: Vec<String>,
}

impl FilePath {
  pub(crate) fn from_prefix_and_path(prefix: &Path, path: &Path) -> Result<FilePath, Error> {
    let relative = path
      .strip_prefix(prefix)
      .context(error::PathStripPrefix { prefix, path })?;

    let mut components = Vec::new();

    for component in relative.components() {
      match component {
        path::Component::Normal(os) => {
          if let Some(unicode) = os.to_str() {
            components.push(unicode.to_owned());
          } else {
            return Err(Error::PathDecode {
              path: relative.to_owned(),
              component: PathBuf::from(component.as_os_str()),
            });
          }
        }
        _ => {
          return Err(Error::PathComponent {
            path: relative.to_owned(),
            component: PathBuf::from(component.as_os_str()),
          })
        }
      }
    }

    if components.is_empty() {
      return Err(Error::PathStripEmpty {
        prefix: prefix.to_owned(),
        path: path.to_owned(),
      });
    }

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
