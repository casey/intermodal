use crate::common::*;

pub(crate) fn env(iter: impl IntoIterator<Item = impl Into<String>>) -> TestEnv {
  TestEnvBuilder::new().arg("imdl").args(iter).build()
}
