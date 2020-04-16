use crate::common::*;

#[derive(Deserialize, Clone)]
pub(crate) struct Example {
  pub(crate) command: String,
  #[serde(default)]
  pub(crate) unstable: bool,
  pub(crate) text: String,
  pub(crate) code: String,
}
