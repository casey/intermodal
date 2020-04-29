use crate::common::*;

#[allow(redundant_semicolons)]
pub(crate) trait CommandExt {
  #[throws]
  fn out(&mut self) -> String;

  #[throws]
  fn status_into_result(&mut self);
}

impl CommandExt for Command {
  #[throws]
  fn out(&mut self) -> String {
    info!("Running {:?}…", self);

    let output = self
      .stdout(Stdio::piped())
      .stderr(Stdio::inherit())
      .output()
      .context(error::CommandInvoke {
        command: format!("{:?}", self),
      })?;

    if !output.status.success() {
      throw!(Error::CommandStatus {
        command: format!("{:?}", self),
        exit_status: output.status,
      });
    }

    let text = String::from_utf8(output.stdout).context(error::CommandDecode {
      command: format!("{:?}", self),
    })?;

    text
  }

  #[throws]
  fn status_into_result(&mut self) {
    let status = self.status().context(error::CommandInvoke {
      command: format!("{:?}", self),
    })?;

    if !status.success() {
      throw!(Error::CommandStatus {
        command: format!("{:?}", self),
        exit_status: status
      });
    }
  }
}
