use std::io::Write;
use std::{fs, io};

use tempfile::{tempdir, TempDir};

pub fn set_up(suffix: &str) -> TempDir {
    let suffix = format!("fs-tests-{}", suffix);
    let test_root_dir = tempdir().expect("failed to create tempdir");
    let test_root = test_root_dir.path();

    match fs::create_dir_all(&test_root) {
        Err(e) => match e.kind() {
            io::ErrorKind::AlreadyExists => {
                write!(
                    io::stderr(),
                    "\nDir '{}' was already present\n",
                    test_root.display()
                )
                .unwrap();
            }
            _ => panic!(e),
        },
        Ok(()) => (),
    }

    test_root_dir
}
