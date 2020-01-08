use crate::common::*;

use std::{io::Cursor, iter};

pub(crate) fn env(iter: impl IntoIterator<Item = impl Into<String>>) -> Env {
  Env::new(
    tempfile::tempdir().unwrap(),
    Cursor::new(Vec::new()),
    iter::once(String::from("imdl")).chain(iter.into_iter().map(|item| item.into())),
  )
}
