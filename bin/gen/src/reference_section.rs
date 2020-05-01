use crate::common::*;

#[derive(Debug, Deserialize, Template)]
#[template(path = "references.md")]
pub(crate) struct ReferenceSection {
  pub(crate) title: String,
  pub(crate) entries: Vec<Reference>,
}

impl ReferenceSection {
  pub(crate) fn path(&self) -> String {
    format!("references/{}.md", self.title.slug())
  }
}
