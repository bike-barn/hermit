use std::io::Write;
use std::path::PathBuf;
use std::{fs, io};

pub fn clean_up(test_root: &PathBuf) {
    if test_root.exists() && test_root.is_dir() {
        match fs::remove_dir_all(test_root) {
            Err(e) => match e.kind() {
                io::ErrorKind::NotFound => {
                    write!(
                        io::stderr(),
                        "\nRemove didn't find dir '{}'",
                        test_root.display()
                    )
                    .unwrap();
                }
                _ => panic!(e),
            },
            Ok(()) => (),
        }
    }
}

pub fn set_up(suffix: &str) -> PathBuf {
    let suffix = format!("fs-tests-{}", suffix);
    let test_root = PathBuf::from("target").join(&suffix);
    clean_up(&test_root);

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

    test_root
}
