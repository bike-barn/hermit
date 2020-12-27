pub use std::fs::*;

#[cfg(windows)]
pub use std::os::windows::fs::symlink_file;

#[cfg(unix)]
pub use std::os::unix::fs::symlink as symlink_file;
