use crate::common::*;

pub(crate) struct Platform;

#[cfg(target_os = "windows")]
impl PlatformInterface for Platform {
  fn hidden(path: &Path) -> Result<bool, Error> {
    use std::os::windows::fs::MetadataExt;

    const HIDDEN_MASK_WIN: u32 = 0x0000_0002;

    let metadata = path.metadata().context(error::Filesystem { path })?;
    Ok((metadata.file_attributes() & HIDDEN_MASK_WIN) != 0)
  }
}

#[cfg(target_os = "macos")]
impl PlatformInterface for Platform {
  fn hidden(path: &Path) -> Result<bool, Error> {
    use std::{ffi::CString, mem, os::unix::ffi::OsStrExt};

    const HIDDEN_MASK_MAC: u32 = 0x0000_8000;

    let mut stat: libc::stat = unsafe { mem::zeroed() };

    let cpath = CString::new(path.as_os_str().as_bytes()).expect("Path contained null character.");

    let error_code = unsafe { libc::stat(cpath.as_ptr(), &mut stat) };

    if error_code != 0 {
      return Err(Error::Filesystem {
        source: io::Error::from_raw_os_error(error_code),
        path: path.to_owned(),
      });
    }

    Ok(stat.st_flags & HIDDEN_MASK_MAC != 0)
  }
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
impl PlatformInterface for Platform {
  fn hidden(_path: &Path) -> Result<bool, Error> {
    Ok(false)
  }
}
