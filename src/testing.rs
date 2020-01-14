use crate::common::*;

pub(crate) fn env(iter: impl IntoIterator<Item = impl Into<String>>) -> TestEnv {
  TestEnv::new(iter)
}
