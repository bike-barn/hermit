use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

pub trait Config {
    fn root_path(&self) -> &PathBuf;

    fn shell_root_path(&self) -> PathBuf {
        self.root_path().join("shells")
    }

    fn current_shell_name(&self) -> Option<String>;

    fn set_current_shell_name(&mut self, name: &str) -> io::Result<()>;

    fn shell_exists(&self, name: &str) -> bool;
}

#[derive(Clone)]
pub struct FsConfig {
    root_path: PathBuf,
    current_shell: Option<String>,
}

fn read_shell_from_path(path: &PathBuf) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut current_shell = String::new();

    file.read_to_string(&mut current_shell)?;

    Ok(current_shell)
}

fn config_path(root_path: &PathBuf) -> PathBuf {
    root_path.join("current_shell")
}

impl FsConfig {
    pub fn new(root_path: impl AsRef<Path>) -> FsConfig {
        let root_path = PathBuf::from(root_path.as_ref());
        let config_path = config_path(&root_path);
        let current_shell = read_shell_from_path(&config_path).ok();

        FsConfig { root_path, current_shell }
    }

    fn config_path(&self) -> PathBuf {
        config_path(&self.root_path())
    }
}

impl Config for FsConfig {
    fn root_path(&self) -> &PathBuf {
        &self.root_path
    }

    fn current_shell_name(&self) -> Option<String> {
        self.current_shell.clone()
    }

    fn set_current_shell_name(&mut self, name: &str) -> io::Result<()> {
        let mut file = File::create(&self.config_path())?;

        file.write_all(name.as_bytes())?;

        self.current_shell = Some(name.to_string());

        Ok(())
    }

    fn shell_exists(&self, name: &str) -> bool {
        let shell_path = self.root_path.join("shells").join(name);
        shell_path.is_dir()
    }
}

#[cfg(test)]
pub mod mock {
    use std::io;
    use std::path::{Path, PathBuf};

    use super::Config;

    #[derive(Clone,Debug,Eq,PartialEq)]
    pub struct MockConfig {
        root_path: PathBuf,
        current_shell: String,
        allowed_shell_names: Vec<String>,
    }

    impl MockConfig {
        pub fn new() -> MockConfig {
            MockConfig {
                root_path: PathBuf::from("/"),
                allowed_shell_names: vec!["default".to_owned()],
                current_shell: "default".to_owned(),
            }
        }

        pub fn with_root(root: impl AsRef<Path>) -> MockConfig {
            MockConfig {
                root_path: PathBuf::from(root.as_ref()),
                allowed_shell_names: vec!["default".to_owned()],
                current_shell: "default".to_owned(),
            }
        }
    }

    impl Config for MockConfig {
        fn root_path(&self) -> &PathBuf {
            &self.root_path
        }

        fn current_shell_name(&self) -> Option<String> {
            Some(self.current_shell.clone())
        }

        fn set_current_shell_name(&mut self, name: &str) -> io::Result<()> {
            self.current_shell = name.to_owned();
            Ok(())
        }

        fn shell_exists(&self, name: &str) -> bool {
            self.allowed_shell_names.contains(&name.to_owned())
        }
    }
}

#[cfg(test)]
mod test {
    use std::fs;
    use std::fs::File;
    use std::path::PathBuf;
    use std::io::prelude::*;
    use super::{Config, FsConfig};

    fn clean_up(test_root: &PathBuf) {
        if test_root.exists() {
            fs::remove_dir_all(test_root).unwrap();
        }
        assert!(!test_root.is_dir());
    }

    fn set_up(suffix: &str, current: &str, shells: Vec<&str>) -> PathBuf {
        let test_root = PathBuf::from("./target/fs-config-tests-".to_owned() + suffix);

        clean_up(&test_root);
        fs::create_dir(&test_root).unwrap();
        assert!(test_root.is_dir());

        let path = test_root.join("current_shell");
        let mut file = File::create(&path).unwrap();
        file.write_all(current.as_bytes()).unwrap();

        let shell_root = test_root.join("shells");
        fs::create_dir(&shell_root).unwrap();
        for shell in shells {
            let new_shell = shell_root.join(PathBuf::from(shell));
            fs::create_dir(&new_shell).unwrap();
        }

        test_root
    }

    #[test]
    fn has_a_root_path() {
        let test_root = set_up("root-path", "default", vec!["default"]);
        let config = FsConfig::new(&test_root);
        assert_eq!(config.root_path(), &test_root);
    }

    #[test]
    fn returns_the_current_shell_name() {
        let test_root = set_up("current-shell-name", "current", vec!["current"]);
        let config = FsConfig::new(&test_root);

        assert_eq!(config.current_shell_name().unwrap(), "current".to_string());
    }

    #[test]
    fn can_set_the_current_shell_name() {
        let test_root = set_up("set-current-shell-name", "default", vec!["default"]);
        let mut config = FsConfig::new(&test_root);
        config.set_current_shell_name("current").unwrap();

        let mut config_file = File::open(&test_root.join("current_shell")).unwrap();
        let mut name_on_disk = String::new();
        config_file.read_to_string(&mut name_on_disk).unwrap();

        let current = "current".to_string();
        assert_eq!(config.current_shell_name().unwrap(), current);
        assert_eq!(name_on_disk, current);
    }

    #[test]
    fn can_confirm_a_shell_exists() {
        let test_root = set_up("confirm-shell-existence",
                               "default",
                               vec!["default", "other"]);
        let config = FsConfig::new(&test_root);

        assert!(config.shell_exists("other"));
    }

    #[test]
    fn can_confirm_a_shell_does_not_exist() {
        let test_root = set_up("confirm-shell-non-existence",
                               "default",
                               vec!["default", "other"]);
        let config = FsConfig::new(&test_root);

        assert!(!config.shell_exists("another"));
    }
}
