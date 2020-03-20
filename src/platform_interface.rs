use crate::common::*;

pub(crate) trait PlatformInterface {
  fn open_file(path: &Path) -> Result<(), Error> {
    Self::open_raw(path.as_os_str())
  }

  fn open_url(url: &Url) -> Result<(), Error> {
    let escaped = Self::escape_url(url);
    Self::open_raw(escaped.as_ref().as_ref())
  }

  fn open_raw(target: &OsStr) -> Result<(), Error> {
    let mut command = Self::opener()?;
    command.push(OsString::from(target));

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

  fn escape_url(url: &Url) -> Cow<str> {
    url.as_str().into()
  }

  fn opener() -> Result<Vec<OsString>, Error>;

  fn hidden(path: &Path) -> Result<bool, Error>;
}
