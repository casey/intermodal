use crate::common::*;

pub(crate) struct Platform;

#[cfg(target_os = "windows")]
impl PlatformInterface for Platform {
  fn opener() -> Result<Vec<OsString>, Error> {
    let exe = if cfg!(test) { "open.bat" } else { "cmd" };
    Ok(vec![
      OsString::from(exe),
      OsString::from("/C"),
      OsString::from("start"),
    ])
  }
}

#[cfg(target_os = "macos")]
impl PlatformInterface for Platform {
  fn opener() -> Result<Vec<OsString>, Error> {
    Ok(vec![OsString::from("open")])
  }
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
impl PlatformInterface for Platform {
  fn opener() -> Result<Vec<OsString>, Error> {
    const OPENERS: &[&str] = &["xdg-open", "gnome-open", "kde-open"];

    for opener in OPENERS {
      if let Ok(output) = Command::new(opener).arg("--version").output() {
        if output.status.success() {
          return Ok(vec![OsString::from(opener)]);
        }
      }
    }

    Err(Error::OpenerMissing { tried: OPENERS })
  }
}
