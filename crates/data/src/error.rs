use crate::common::*;

#[derive(Debug, PartialEq)]
pub enum Error {
  Alignment { alignment: usize, offset: usize },
  UnicodeDecode { source: Utf8Error },
  Size { have: usize, want: usize },
}
