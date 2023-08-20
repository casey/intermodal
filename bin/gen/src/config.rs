use crate::common::*;

const PATH: &str = "bin/gen/config.yaml";

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
  pub(crate) examples: Vec<Example>,
  pub(crate) packages: Vec<Package>,
  pub(crate) references: Vec<ReferenceSection>,
  pub(crate) faq: Vec<FaqEntry>,
}

impl Config {
  #[throws]
  pub(crate) fn load(root: &Path) -> Config {
    let path = root.join(PATH);
    let file = File::open(&path).context(error::Filesystem { path: &path })?;
    serde_yaml::from_reader(file).context(error::ConfigDeserialize { path })?
  }
}
