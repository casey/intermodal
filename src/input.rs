use crate::common::*;

pub(crate) struct Input {
  source: InputTarget,
  data: Vec<u8>,
}

impl Input {
  pub(crate) fn data(&self) -> &[u8] {
    &self.data
  }

  pub(crate) fn source(&self) -> &InputTarget {
    &self.source
  }
}
