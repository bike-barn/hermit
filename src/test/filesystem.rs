use std::{fs, io};
use std::io::Write;
use std::path::PathBuf;

use uuid::Uuid;

pub fn clean_up(test_root: &PathBuf) {
    if test_root.exists() && test_root.is_dir() {
        match fs::remove_dir_all(test_root) {
            Err(e) => {
                match e.kind() {
                    io::ErrorKind::NotFound => {
                        write!(io::stderr(),
                               "\nRemove didn't find dir '{}'",
                               test_root.display())
                            .unwrap();
                    }
                    _ => panic!(e),
                }
            }
            Ok(()) => (),
        }
    }
}

pub fn set_up(suffix: &str) -> PathBuf {
    let random_uuid = Uuid::new_v4();
    let suffix = format!("{}-{}", suffix, random_uuid);
    let test_root = PathBuf::from("target").join(&suffix);
    clean_up(&test_root);

    match fs::create_dir_all(&test_root) {
        Err(e) => {
            match e.kind() {
                io::ErrorKind::AlreadyExists => {
                    write!(io::stderr(),
                           "\nDir '{}' was already present\n",
                           test_root.display())
                        .unwrap();
                }
                _ => panic!(e),
            }
        }
        Ok(()) => (),
    }

    test_root
}
