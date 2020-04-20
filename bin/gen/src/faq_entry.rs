use crate::common::*;

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct FaqEntry {
  pub(crate) title: String,
  pub(crate) text: String,
  pub(crate) anchor: String,
}
