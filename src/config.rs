use std::fs::File;
use std::error::Error;
use std::io::prelude::*;
use std::path::PathBuf;

pub trait Config {
    fn root_path(&self) -> &PathBuf;

    fn current_shell_name(&self) -> String;

    fn set_current_shell_name(&mut self, name: &str); // TODO: Consider making this trait return `Result<(), Error>`.

    fn does_shell_exist(&self, name: &str) -> bool;
}

pub struct FilesystemConfig {
    pub root_path: PathBuf,
    pub current_shell: String,
}

impl FilesystemConfig {
    // TODO: Make sure this actually works.
    fn new(root_path: PathBuf) -> Self {
        let config_path = root_path.join("current_shell");
        let config_display = config_path.display();

        let mut file = match File::open(&config_path) {
            Ok(file) => file,
            Err(e) => panic!("couldn't open {}: {}", config_display, e),
        };

        let mut current_shell = String::new();
        match file.read_to_string(&mut current_shell) {
            Ok(_) => {},
            Err(e) => panic!("error: couldn't read {}: {}", config_display, e),
        }

        FilesystemConfig {
            root_path: root_path,
            current_shell: current_shell,
        }
    }

}

impl Config for FilesystemConfig {
    fn root_path(&self) -> &PathBuf {
        &self.root_path
    }

    // TODO: Does this actually need to read from the filesystem as well?
    fn current_shell_name(&self) -> String {
        self.current_shell.clone()
    }

    fn set_current_shell_name(&mut self, name: &str) {
        // TODO: Write value of self.current_shell to `name` and
        // update self.current_shell. This _might_ fail...
        ()
    }

    fn does_shell_exist(&self, name: &str) -> bool {
        // TODO: Check to see if a dir by `name` exists in
        // self.root_path/shells. return true if exists, false otherwise.
        true
    }
}

#[cfg(test)]
pub mod mock {
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

        fn set_current_shell_name(&mut self, name: &str) {
            self.current_shell = name.to_string();
        }

        fn does_shell_exist(&self, name: &str) -> bool {
            self.allowed_shell_names.contains(&name.to_string())
        }
    }
}

#[cfg(test)]
mod test {
    use super::FilesystemConfig;

    // TODO: Figure out the best way to do FS setup/teardown in Rust.
    /*
    fn setup_test_dir() {
    }

    fn teardown_test_dir() {
    }
    */

    // TODO: Make sure this actually works.
    #[test]
    fn has_a_root_path() {
        let root_path = "./fs-config-test";
        let config = FilesystemConfig::new(PathBuf::from(root_path));
        assert_eq!(config.root_path(), "./fs-config-test")
    }

    // TODO: Actually implement the rest of the test methods.
    /*
    #[test]
    fn returns_the_current_shell_name() {
    }

    #[test]
    fn can_set_the_current_shell_name() {
    }

    #[test]
    fn can_confirm_a_shell_exists() {
    }

    #[test]
    fn can_confirm_a_shell_does_not_exist() {
    }
    */
}
