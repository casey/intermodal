use crate::common::*;

pub(crate) trait ExitStatusExt {
  fn into_result(self) -> anyhow::Result<()>;
}

impl ExitStatusExt for ExitStatus {
  #[throws]
  fn into_result(self) {
    if !self.success() {
      throw!(anyhow!(self));
    }
  }
}
