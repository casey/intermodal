use crate::common::*;

#[derive(Template)]
#[template(path = "introduction.md")]
pub(crate) struct Introduction {
  pub(crate) examples: Vec<Example>,
}

impl Introduction {
  pub(crate) fn new(config: &Config) -> Self {
    Self {
      examples: config.examples.clone(),
    }
  }
}
