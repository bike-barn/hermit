use config::HermitConfig;
use shell::Shell;
use std::path::PathBuf;

struct Hermit<T: HermitConfig> {
    root_path: PathBuf,
    config: T,
}

impl<T: HermitConfig> Hermit<T> {
    fn current_shell(&self) -> Shell {
        Shell::new(self.config.current_shell_name(), &self.root_path)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use config::mock::MockConfig;

    use super::Hermit;

    fn hermit(config: &MockConfig) -> Hermit<MockConfig> {
        Hermit {
            root_path: PathBuf::from("/"),
            config: config.clone(),
        }
    }

    #[test]
    fn returns_the_default_shell() {
        let config = MockConfig { default_shell: "default".to_string() };
        let hermit = hermit(&config);
        let shell = hermit.current_shell();
        assert_eq!(shell.name, "default");
        assert_eq!(*shell.root_path, hermit.root_path);
    }

    #[test]
    fn returns_the_current_shell() {
        let config = MockConfig { default_shell: "current".to_string() };
        let hermit = hermit(&config);
        let shell = hermit.current_shell();
        assert_eq!(shell.name, "current");
        assert_eq!(*shell.root_path, hermit.root_path);
    }
}
