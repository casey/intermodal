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
    use std::os::macos::fs::MetadataExt;

    const HIDDEN_MASK_MAC: u32 = 0x0000_8000;

    let metadata = path.metadata().context(error::Filesystem { path })?;

    Ok(metadata.st_flags() & HIDDEN_MASK_MAC != 0)
  }
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
impl PlatformInterface for Platform {
  fn hidden(_path: &Path) -> Result<bool, Error> {
    Ok(false)
  }
}

#[cfg(tests)]
mod tests {
  use super::*;

  #[test]
  fn hidden() {
    let tmp = tempdir().unwrap();

    let file = tmp.path().join("file");

    assert!(!Platform::hidden(&file));

    if cfg!(target_os = "windows") {
      Command::new("attrib")
        .arg("+h")
        .arg(&file)
        .status()
        .unwrap();
    } else if cfg!(target_os = "macos") {
      Command::new("chflags")
        .arg("hidden")
        .arg(&file)
        .status()
        .unwrap();
    } else {
      return;
    }

    assert!(Platform::hidden(&file));
  }
}
