use crate::common::*;

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Example {
  pub(crate) command: String,
  #[serde(default)]
  pub(crate) unstable: bool,
  pub(crate) text: String,
  pub(crate) code: String,
}
