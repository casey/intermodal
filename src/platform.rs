use crate::common::*;

pub(crate) struct Platform;

#[cfg(target_os = "windows")]
impl PlatformInterface for Platform {}

#[cfg(target_os = "macos")]
impl PlatformInterface for Platform {}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
impl PlatformInterface for Platform {}
