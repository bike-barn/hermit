use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;

pub trait Config {
    fn root_path(&self) -> &PathBuf;

    fn current_shell_name(&self) -> String;

    fn set_current_shell_name(&mut self, name: &str) -> io::Result<()>;

    fn does_shell_exist(&self, name: &str) -> io::Result<bool>;
}

#[derive(Clone)]
pub struct FsConfig {
    pub root_path: PathBuf,
    pub current_shell: String,
}

impl FsConfig {
    fn new(root_path: PathBuf) -> io::Result<Self> {
        let config_path = root_path.join("current_shell");
        let config_display = config_path.display();

        let mut file = try!(File::open(&config_path));
        let mut current_shell = String::new();

        try!(file.read_to_string(&mut current_shell));

        Ok(FsConfig {
            root_path: root_path,
            current_shell: current_shell,
        })
    }
}

impl Config for FsConfig {
    fn root_path(&self) -> &PathBuf {
        &self.root_path
    }

    // TODO: Does this actually need to read from the filesystem as well?
    fn current_shell_name(&self) -> String {
        self.current_shell.clone()
    }

    fn set_current_shell_name(&mut self, name: &str) -> io::Result<()> {
        let config_path = self.root_path.join("current_shell");
        let config_display = config_path.display();

        let mut file = try!(File::create(&config_path));

        try!(file.write_all(self.current_shell.as_bytes()));

        self.current_shell = name.to_string();
        Ok(())
    }

    fn does_shell_exist(&self, name: &str) -> io::Result<bool> {
        // TODO: Check to see if a dir by `name` exists in
        // self.root_path/shells. return true if exists, false otherwise.
        let shell_root = self.root_path.join("shells");
        Ok(try!(fs::read_dir(shell_root)).any(|entry| {
            let entry = try!(entry);
            entry.path().is_dir() && match entry.file_name().to_str() {
                Some(n) => n == name,
                None => false
            }
        }))
    }
}

#[cfg(test)]
pub mod mock {
    use std::io;
    use std::path::PathBuf;

    use super::Config;

    #[derive(Clone)]
    pub struct MockConfig {
        pub root_path: PathBuf,
        pub current_shell: String,
        pub allowed_shell_names: Vec<String>,
    }

    impl Config for MockConfig {
        fn root_path(&self) -> &PathBuf {
            &self.root_path
        }

        fn current_shell_name(&self) -> String {
            self.current_shell.clone()
        }

        fn set_current_shell_name(&mut self, name: &str) -> io::Result<()> {
            self.current_shell = name.to_string();
            Ok(())
        }

        fn does_shell_exist(&self, name: &str) -> io::Result<bool> {
            Ok(self.allowed_shell_names.contains(&name.to_string()))
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
        let config = FsConfig::new(test_root.clone()).unwrap();
        assert_eq!(config.root_path(), &test_root);

        clean_up(&test_root);
    }

    #[test]
    fn returns_the_current_shell_name() {
        let test_root = set_up("current-shell-name", "current", vec!["default"]);
        let config = FsConfig::new(test_root.clone()).unwrap();
        assert_eq!(config.current_shell_name(), "current".to_string());

        clean_up(&test_root);
    }

    #[test]
    fn can_set_the_current_shell_name() {
        let test_root = set_up("set-current-shell-name", "default", vec!["default"]);
        let mut config = FsConfig::new(test_root.clone()).unwrap();
        config.set_current_shell_name("current");
        assert_eq!(config.current_shell_name(), "current".to_string());

        clean_up(&test_root);
    }

    #[test]
    fn can_confirm_a_shell_exists() {
        let test_root = set_up("confirm-shell-existence",
                               "default",
                               vec!["default", "other"]);
        let mut config = FsConfig::new(test_root.clone()).unwrap();

        let shell_exists = config.does_shell_exist("other");
        assert!(shell_exists.is_ok());
        assert!(shell_exists.unwrap());

        clean_up(&test_root);
    }

    // TODO: Actually implement the rest of the test methods.
    //
    // #[test]
    // fn can_confirm_a_shell_does_not_exist() {
    // }
    //

}
