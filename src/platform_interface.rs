use crate::common::*;

pub(crate) trait PlatformInterface {
  fn open_file(path: &Path) -> Result<(), Error> {
    Self::open_target(path.as_ref())
  }

  fn open_url(url: &Url) -> Result<(), Error> {
    Self::open_target(url.as_str().as_ref())
  }

  fn open_target(target: &OsStr) -> Result<(), Error> {
    open::that(target).context(error::OpenerInvoke)
  }
}
