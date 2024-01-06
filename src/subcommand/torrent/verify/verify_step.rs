use crate::common::*;

#[derive(Clone, Copy)]
pub(crate) enum VerifyStep<'a> {
  Loading { metainfo: &'a InputTarget },
  Verifying { content: &'a Path },
}

impl<'a> Step for VerifyStep<'a> {
  fn n(&self) -> usize {
    match self {
      Self::Loading { .. } => 1,
      Self::Verifying { .. } => 2,
    }
  }

  fn symbol(&self) -> &str {
    match self {
      Self::Loading { .. } => "\u{1F4BE}",
      Self::Verifying { .. } => "\u{1F9EE}",
    }
  }

  fn total() -> usize {
    2
  }

  fn write_message(&self, write: &mut dyn Write) -> io::Result<()> {
    match self {
      Self::Loading { metainfo } => write!(write, "Loading metainfo from {metainfo}…"),
      Self::Verifying { content } => {
        write!(write, "Verifying pieces from `{}`…", content.display())
      }
    }
  }
}
