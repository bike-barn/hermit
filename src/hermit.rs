use config::HermitConfig;
use shell::Shell;
use std::path::PathBuf;

struct Hermit<T: HermitConfig> {
    root_path: PathBuf,
    config: T,
}

#[derive(Copy,Clone)]
#[derive(PartialEq,Eq)]
#[derive(Debug)]
pub enum Error {
    ShellDoesNotExist,
}

impl<T: HermitConfig> Hermit<T> {
    fn current_shell(&self) -> Shell {
        Shell::new(self.config.current_shell_name(), &self.root_path)
    }

    fn set_current_shell(&mut self, name: &str) -> Result<(), Error> {
        if self.config.does_shell_exist(name) {
            self.config.set_current_shell_name(name);
            Ok(())
        } else {
            Err(Error::ShellDoesNotExist)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use config::mock::MockConfig;

    use super::{Error, Hermit};

    fn hermit(config: &MockConfig) -> Hermit<MockConfig> {
        Hermit {
            root_path: PathBuf::from("/"),
            config: config.clone(),
        }
    }

    #[test]
    fn returns_the_default_shell() {
        let config = MockConfig {
            allowed_shell_names: vec!["default".to_string()],
            default_shell: "default".to_string(),
        };
        let hermit = hermit(&config);
        let shell = hermit.current_shell();
        assert_eq!(shell.name, "default");
        assert_eq!(*shell.root_path, hermit.root_path);
    }

    #[test]
    fn returns_the_current_shell() {
        let config = MockConfig {
            allowed_shell_names: vec!["default".to_string()],
            default_shell: "current".to_string(),
        };
        let hermit = hermit(&config);
        let shell = hermit.current_shell();
        assert_eq!(shell.name, "current");
        assert_eq!(*shell.root_path, hermit.root_path);
    }

    #[test]
    fn can_set_the_current_shell() {
        let config = MockConfig {
            allowed_shell_names: vec!["default".to_string()],
            default_shell: "current".to_string(),
        };
        let mut hermit = hermit(&config);

        assert_eq!(hermit.current_shell().name, "current");
        assert!(hermit.set_current_shell("default").is_ok());
        assert_eq!(hermit.current_shell().name, "default");
    }

    #[test]
    fn cant_set_the_current_shell_to_a_nonexistent_shell() {
        let config = MockConfig {
            allowed_shell_names: vec!["default".to_string()],
            default_shell: "current".to_string(),
        };
        let mut hermit = hermit(&config);

        assert_eq!(hermit.current_shell().name, "current");
        let res = hermit.set_current_shell("non-existent");
        assert!(res.is_err());
        assert_eq!(res.err().unwrap(), Error::ShellDoesNotExist);
    }
}
