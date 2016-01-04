use config::HermitConfig;
use shell::Shell;

struct Hermit<T: HermitConfig> {
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
        Shell::new(self.config.current_shell_name(), self.config.root_path())
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

    use config::HermitConfig;
    use config::mock::MockConfig;

    use super::{Error, Hermit};

    fn hermit(config: &MockConfig) -> Hermit<MockConfig> {
        Hermit { config: config.clone() }
    }

    fn mock_config() -> MockConfig {
        MockConfig {
            root_path: PathBuf::from("/"),
            allowed_shell_names: vec!["default".to_string()],
            current_shell: "default".to_string(),
        }
    }

    #[test]
    fn returns_the_current_shell() {
        let config = mock_config();
        let hermit = hermit(&config);

        let shell = hermit.current_shell();
        assert_eq!(shell.name, "default");
        assert_eq!(shell.root_path, config.root_path());
    }

    #[test]
    fn can_set_the_current_shell() {
        let mut config = mock_config();
        config.current_shell = "current".to_string();
        let mut hermit = hermit(&config);

        assert_eq!(hermit.current_shell().name, "current");
        assert!(hermit.set_current_shell("default").is_ok());
        assert_eq!(hermit.current_shell().name, "default");
    }

    #[test]
    fn cant_set_the_current_shell_to_a_nonexistent_shell() {
        let config = mock_config();
        let mut hermit = hermit(&config);

        assert_eq!(hermit.current_shell().name, "default");
        let res = hermit.set_current_shell("non-existent");
        assert!(res.is_err());
        assert_eq!(res.err().unwrap(), Error::ShellDoesNotExist);
    }
}
