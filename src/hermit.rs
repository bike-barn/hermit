use std::io;
use config::Config;
use file_operations::FileOperations;
use shell::Shell;

#[derive(Copy,Clone)]
#[derive(PartialEq,Eq)]
#[derive(Debug)]
pub enum Error {
    ShellDoesNotExist,
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::ShellDoesNotExist
    }
}

pub struct Hermit<T: Config> {
    config: T,
}

impl<T: Config> Hermit<T> {
    pub fn new(config: T) -> Hermit<T> {
        Hermit { config: config }
    }

    pub fn current_shell(&self) -> Option<Shell> {
        self.config
            .current_shell_name()
            .map(|shell_name| Shell::new(shell_name, self.config.root_path()))
    }

    pub fn set_current_shell(&mut self, name: &str) -> Result<(), Error> {
        if self.config.does_shell_exist(name) {
            self.config.set_current_shell_name(name).map_err(From::from)
        } else {
            Err(Error::ShellDoesNotExist)
        }
    }

    pub fn init_shell(&self, file_ops: &mut FileOperations, name: &str) {
        let path = self.config.shell_root_path().join(name);
        file_ops.create_git_repo(path);
    }

    pub fn inhabit_shell(&mut self,
                         _file_ops: &mut FileOperations,
                         name: &str)
                         -> Result<(), Error> {
        return self.set_current_shell(name);
    }

    pub fn shell_list(&mut self) -> Result<Vec<String>, io::Error> {
        return self.config.get_shell_list();
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use config::Config;
    use config::mock::MockConfig;
    use file_operations::FileOperations;
    use file_operations::Op;

    use super::{Error, Hermit};

    fn hermit(config: &MockConfig) -> Hermit<MockConfig> {
        Hermit { config: config.clone() }
    }

    fn mock_config() -> MockConfig {
        MockConfig {
            root_path: PathBuf::from("/"),
            allowed_shell_names: vec!["default".to_owned()],
            current_shell: "default".to_owned(),
        }
    }

    #[test]
    fn returns_the_current_shell() {
        let config = mock_config();
        let hermit = hermit(&config);

        let shell = hermit.current_shell().unwrap();
        assert_eq!(shell.name, "default");
        assert_eq!(shell.root_path, config.root_path());
    }

    #[test]
    fn can_set_the_current_shell() {
        let mut config = mock_config();
        config.current_shell = "current".to_owned();
        let mut hermit = hermit(&config);

        assert_eq!(hermit.current_shell().unwrap().name, "current");
        assert!(hermit.set_current_shell("default").is_ok());
        assert_eq!(hermit.current_shell().unwrap().name, "default");
    }

    #[test]
    fn cant_set_the_current_shell_to_a_nonexistent_shell() {
        let config = mock_config();
        let mut hermit = hermit(&config);

        assert_eq!(hermit.current_shell().unwrap().name, "default");
        let res = hermit.set_current_shell("non-existent");
        assert!(res.is_err());
        assert_eq!(res, Err(Error::ShellDoesNotExist));
    }

    #[test]
    fn can_initialize_a_new_shell() {
        let config = MockConfig {
            root_path: PathBuf::from(".hermit-config"),
            allowed_shell_names: vec!["default".to_owned()],
            current_shell: "default".to_owned(),
        };
        let hermit = hermit(&config);
        let mut file_ops = FileOperations::rooted_at("/home/geoff");

        hermit.init_shell(&mut file_ops, "new-one");
        let first_op = &file_ops.operations[0];
        assert_eq!(*first_op,
                   Op::GitInit(PathBuf::from("/home/geoff/.hermit-config/shells/new-one")));
    }

    #[test]
    fn can_inhabit_existing_shell() {
        let config = MockConfig {
            root_path: PathBuf::from(".config"),
            allowed_shell_names: vec!["default".to_owned(), "new-one".to_owned()],
            current_shell: "default".to_owned(),
        };
        let mut hermit = hermit(&config);
        let mut file_ops = FileOperations::rooted_at("/home/houngj");
        hermit.inhabit_shell(&mut file_ops, "new-one");
        let shell = hermit.current_shell().expect("current shell does not exist. Check that \
                                                   'hermit.inhabit_shell()' is functioning \
                                                   properly");
        assert_eq!(shell.name, "new-one");
    }

    #[test]
    fn cant_inhabit_nonexisting_shell() {
        let config = MockConfig {
            root_path: PathBuf::from(".config"),
            allowed_shell_names: vec!["default".to_owned()],
            current_shell: "default".to_owned(),
        };
        let mut hermit = hermit(&config);
        let mut file_ops = FileOperations::rooted_at("/home/houngj");
        let res = hermit.inhabit_shell(&mut file_ops, "not-a-shell");
        assert!(res.is_err());
        assert_eq!(res, Err(Error::ShellDoesNotExist));
    }
}
