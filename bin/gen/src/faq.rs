use crate::common::*;

#[derive(Template)]
#[template(path = "faq.md")]
pub(crate) struct Faq {
  pub(crate) entries: Vec<FaqEntry>,
}

impl Faq {
  pub(crate) fn new(entries: &[FaqEntry]) -> Self {
    Self {
      entries: entries.to_vec(),
    }
  }
}
