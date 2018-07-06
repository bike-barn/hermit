use std::{io, result};
use std::rc::Rc;

use config::Config;
use file_operations::FileOperations;
use message;
use shell::Shell;

#[derive(Clone, Debug, Fail, PartialEq, Eq)]
pub enum Error {
    #[fail(display = "{} subcommand has not been implemented yet", _0)]
    SubcommandNotImplemented(&'static str),

    #[fail(display = "That is not the name of a shell")]
    ShellDoesNotExist,
}

impl From<io::Error> for Error {
    fn from(_err: io::Error) -> Error {
        Error::ShellDoesNotExist
    }
}

pub type Result = result::Result<(), Error>;

pub struct Hermit<T: Config> {
    config: Rc<T>,
}

impl<T: Config> Hermit<T> {
    pub fn new(config: T) -> Hermit<T> {
        Hermit { config: Rc::new(config) }
    }

    pub fn current_shell(&self) -> Option<Shell<T>> {
        self.config
            .current_shell_name()
            .map(|shell_name| Shell::new(shell_name, self.config.clone()))
    }

    pub fn set_current_shell(&mut self, name: &str) -> Result {
        if self.config.shell_exists(name) {
            match Rc::get_mut(&mut self.config) {
                Some(config) => config.set_current_shell_name(name).map_err(Error::from),
                None => {
                    unreachable!(message::error_str("attempted to modify config while it was being used."))
                }
            }
        } else {
            Err(Error::ShellDoesNotExist)
        }
    }

    pub fn init_shell(&self, file_ops: &mut FileOperations, name: &str) {
        let new_shell = Shell::new(name, self.config.clone());
        let path = new_shell.root_path();
        file_ops.create_git_repo(path);
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::rc::Rc;

    use config::Config;
    use config::mock::MockConfig;
    use file_operations::FileOperations;
    use file_operations::Op;

    use super::{Error, Hermit};

    fn hermit(config: &MockConfig) -> Hermit<MockConfig> {
        Hermit::new(config.clone())
    }

    #[test]
    fn returns_the_current_shell() {
        let config = MockConfig::new();
        let hermit = hermit(&config);

        let shell = hermit.current_shell().unwrap();
        assert_eq!(shell.name, "default");
        assert_eq!(shell.config, Rc::new(config));
    }

    #[test]
    fn can_set_the_current_shell() {
        let mut config = MockConfig::new();
        config.set_current_shell_name("current").expect("Setting shell name failed");
        let mut hermit = hermit(&config);

        assert_eq!(hermit.current_shell().unwrap().name, "current");
        hermit.set_current_shell("default").expect("Setting shell name failed");
        assert_eq!(hermit.current_shell().unwrap().name, "default");
    }

    #[test]
    fn cant_set_the_current_shell_to_a_nonexistent_shell() {
        let config = MockConfig::new();
        let mut hermit = hermit(&config);

        assert_eq!(hermit.current_shell().unwrap().name, "default");
        let res = hermit.set_current_shell("non-existent");
        let err = res.expect_err("Shell should not exist");
        assert_eq!(err, Error::ShellDoesNotExist);
    }

    #[test]
    fn can_initialize_a_new_shell() {
        let config = MockConfig::with_root(".hermit-config", );
        let hermit = hermit(&config);
        let mut file_ops = FileOperations::rooted_at("/home/geoff");

        hermit.init_shell(&mut file_ops, "new-one");
        let first_op = &file_ops.operations()[0];
        assert_eq!(*first_op,
                   Op::GitInit(PathBuf::from("/home/geoff/.hermit-config/shells/new-one")));
    }
}
