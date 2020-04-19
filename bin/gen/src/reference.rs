use crate::common::*;

#[derive(Debug, Deserialize)]
pub(crate) struct Reference {
  pub(crate) url: Url,
  pub(crate) title: Option<String>,
  pub(crate) description: String,
}
