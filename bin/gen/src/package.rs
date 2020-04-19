use crate::common::*;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct Package {
  operating_system: String,
  package_manager: String,
  package: String,
  command: String,
}

impl Row for Package {
  fn header() -> &'static [&'static str] {
    &["Operating System", "Package Manager", "Package", "Command"]
  }

  fn entries(&self) -> Vec<&str> {
    vec![
      &self.operating_system,
      &self.package_manager,
      &self.package,
      &self.command,
    ]
  }
}
