use crate::common::*;

#[derive(Clone, Copy)]
pub(crate) enum CreateStep<'output> {
  Searching,
  Hashing,
  Writing { output: &'output OutputTarget },
}

impl<'output> Step for CreateStep<'output> {
  fn n(&self) -> usize {
    match self {
      Self::Searching => 1,
      Self::Hashing => 2,
      Self::Writing { .. } => 3,
    }
  }

  fn symbol(&self) -> &str {
    match self {
      Self::Searching => "\u{1F9FF}",
      Self::Hashing => "\u{1F9EE}",
      Self::Writing { .. } => "\u{1F4BE}",
    }
  }

  fn total() -> usize {
    3
  }

  fn write_message(&self, write: &mut dyn Write) -> io::Result<()> {
    match self {
      Self::Searching => write!(write, "Searching for files…"),
      Self::Hashing => write!(write, "Hashing pieces…"),
      Self::Writing { output } => write!(write, "Writing metainfo to {}…", output),
    }
  }
}
