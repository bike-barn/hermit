use std::rc::Rc;
use std::{io, result};

use crate::config::Config;
use crate::file_operations::FileOperations;
use crate::message;
use crate::shell::Shell;

#[derive(Clone, Debug, Fail, PartialEq, Eq)]
pub enum Error {
    #[fail(display = "{} subcommand has not been implemented yet", _0)]
    SubcommandNotImplemented(&'static str),

    #[fail(display = "That is not the name of a shell")]
    ShellDoesNotExist,

    #[fail(display = "No shell is active right now")]
    NoActiveShell,
}

impl From<io::Error> for Error {
    fn from(_err: io::Error) -> Error {
        Error::ShellDoesNotExist
    }
}

pub type Result<T> = result::Result<T, Error>;

pub struct Hermit<T: Config> {
    config: Rc<T>,
}

impl<T: Config> Hermit<T> {
    pub fn new(config: T) -> Hermit<T> {
        Hermit {
            config: Rc::new(config),
        }
    }

    pub fn current_shell(&self) -> Result<Shell<T>> {
        self.config
            .current_shell_name()
            .map(|shell_name| Shell::new(shell_name, self.config.clone()))
            .ok_or(Error::NoActiveShell)
    }

    fn set_current_shell(&mut self, name: &str) -> Result<()> {
        match Rc::get_mut(&mut self.config) {
            Some(config) => config.set_current_shell_name(name).map_err(Error::from),
            None => unreachable!(message::error_str(
                "attempted to modify config while it was being used."
            )),
        }
    }

    pub fn init_shell(&mut self, file_ops: &mut FileOperations, name: &str) -> Result<()> {
        self.set_current_shell(name)?;
        let new_shell = self.current_shell()?;
        let path = new_shell.root_path();
        let parent = path.parent().expect("Shell root path was too short");
        file_ops.create_dir(parent);
        file_ops.create_git_repo(&path);
        Ok(())
    }

    pub fn inhabit(&mut self, file_ops: &mut FileOperations, name: &str) -> Result<()> {
        if self.config.shell_exists(name) {
            if let Ok(shell) = self.current_shell() {
                shell.unlink(file_ops)
            }

            self.set_current_shell(name)?;

            if let Ok(shell) = self.current_shell() {
                shell.link(file_ops)
            }
            Ok(())
        } else {
            Err(Error::ShellDoesNotExist)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::ops::*;

    use std::path::PathBuf;
    use std::rc::Rc;

    use crate::config::mock::MockConfig;
    use crate::config::Config;
    use crate::file_operations::FileOperations;
    use crate::file_operations::Op;

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
        config
            .set_current_shell_name("current")
            .expect("Setting shell name failed");
        let mut hermit = hermit(&config);

        assert_eq!(hermit.current_shell().unwrap().name, "current");
        hermit
            .set_current_shell("default")
            .expect("Setting shell name failed");
        assert_eq!(hermit.current_shell().unwrap().name, "default");
    }

    #[test]
    fn can_initialize_a_new_shell() {
        let config = MockConfig::with_root(".hermit-config");
        let mut hermit = hermit(&config);
        let mut file_ops = FileOperations::rooted_at("/home/geoff");

        hermit
            .init_shell(&mut file_ops, "new-one")
            .expect("Init shell failed");
        let first_op = &file_ops.operations()[0];
        assert_eq!(
            *first_op,
            Op::MkDir(PathBuf::from("/home/geoff/.hermit-config/shells"))
        );
        let second_op = &file_ops.operations()[1];
        assert_eq!(
            *second_op,
            Op::GitInit(PathBuf::from("/home/geoff/.hermit-config/shells/new-one"))
        );
    }

    #[test]
    fn can_inhabit_and_change_shells() {
        let hermit_root = PathBuf::from(".hermit-config");
        let mut config = MockConfig::with_root(&hermit_root);
        config.set_paths(vec![".bashrc", ".boot/profile.boot"]);
        let mut hermit = hermit(&config);
        let op_root_path = PathBuf::from("/home/geoff");
        let mut file_ops = FileOperations::rooted_at(&op_root_path);

        hermit
            .inhabit(&mut file_ops, "default")
            .expect("Inhabit failed");

        let new_shell_root = hermit_root.join("shells/default");
        assert_eq!(
            file_ops.operations(),
            &vec![
                Op::Remove(op_root_path.join(".bashrc")),
                Op::Remove(op_root_path.join(".boot/profile.boot")),
                link_op_for(&new_shell_root, &op_root_path, ".bashrc"),
                link_op_for(&new_shell_root, &op_root_path, ".boot/profile.boot"),
            ]
        );
    }
}
