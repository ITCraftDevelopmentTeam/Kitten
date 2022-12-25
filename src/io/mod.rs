#[cfg(target_os = "windows")]
use {windows::*,windows_sys::*};

#[cfg(target_os = "linux")]
use libc::*;

#[cfg(target_os = "macos")]
use libc::*;

pub mod fs;
pub mod mem;