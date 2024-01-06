use crate::common::*;

#[derive(Clone, Copy)]
pub(crate) enum CreateStep<'a> {
  Searching { input: &'a InputTarget },
  Hashing,
  Writing { output: &'a OutputTarget },
}

impl<'a> Step for CreateStep<'a> {
  fn n(&self) -> usize {
    match self {
      Self::Searching { .. } => 1,
      Self::Hashing => 2,
      Self::Writing { .. } => 3,
    }
  }

  fn symbol(&self) -> &str {
    match self {
      Self::Searching { .. } => "\u{1F9FF}",
      Self::Hashing => "\u{1F9EE}",
      Self::Writing { .. } => "\u{1F4BE}",
    }
  }

  fn total() -> usize {
    3
  }

  fn write_message(&self, write: &mut dyn Write) -> io::Result<()> {
    match self {
      Self::Searching { input } => match input {
        InputTarget::Path(path) => write!(write, "Searching `{}` for files…", path.display()),
        InputTarget::Stdin => write!(write, "Creating single-file torrent from standard input…"),
      },

      Self::Hashing => write!(write, "Hashing pieces…"),
      Self::Writing { output } => write!(write, "Writing metainfo to {output}…"),
    }
  }
}
