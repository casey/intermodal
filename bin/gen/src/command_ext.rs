use crate::common::*;

pub(crate) trait CommandExt {
  #[throws]
  fn out(&mut self) -> String;
}

impl CommandExt for Command {
  #[throws]
  fn out(&mut self) -> String {
    info!("Running {:?}…", self);

    let output = self
      .stdout(Stdio::piped())
      .stderr(Stdio::inherit())
      .output()?;

    output.status.into_result()?;

    let text = String::from_utf8(output.stdout)?;

    text
  }
}
