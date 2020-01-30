use crate::common::*;

pub(crate) trait PlatformInterface {
  fn open(path: &Path) -> Result<(), Error> {
    let mut command = Self::opener()?;
    command.push(OsString::from(path));

    let command_string = || {
      command
        .iter()
        .map(|arg| arg.to_string_lossy().into_owned())
        .collect::<Vec<String>>()
        .join(",")
    };

    let status = Command::new(&command[0])
      .args(&command[1..])
      .status()
      .map_err(|source| Error::CommandInvoke {
        source,
        command: command_string(),
      })?;

    if status.success() {
      Ok(())
    } else {
      Err(Error::CommandStatus {
        command: command_string(),
        status,
      })
    }
  }

  fn opener() -> Result<Vec<OsString>, Error>;
}
