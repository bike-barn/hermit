use std::path::PathBuf;
use std::rc::Rc;

use config::Config;

pub struct Shell<T: Config> {
    pub name: String,
    pub config: Rc<T>,
}

impl<T: Config> Shell<T> {
    pub fn new<S>(name: S, config: Rc<T>) -> Shell<T>
        where S: Into<String>
    {
        Shell {
            name: name.into(),
            config: config,
        }
    }

    pub fn root_path(&self) -> PathBuf {
        self.config.shell_root_path().join(&self.name)
    }

    pub fn path_for(&self, filename: &str) -> PathBuf {
        self.root_path().join(filename)
    }
}

#[cfg(test)]
mod tests {

    use std::path::{Path, PathBuf};
    use std::rc::Rc;

    use config::mock::MockConfig;

    use super::Shell;

    fn root_path(path_str: &str) -> PathBuf {
        PathBuf::from(path_str)
    }

    fn mock_config<P: AsRef<Path>>(root_path: P) -> Rc<MockConfig> {
        Rc::new(MockConfig::with_root(root_path))
    }

    #[test]
    fn has_a_name() {
        let config = mock_config("/");
        let s = Shell::new("my_shell", config);
        assert_eq!(s.name, "my_shell");
    }

    #[test]
    fn has_a_string_name() {
        let config = mock_config("/");
        let s = Shell::new(String::from("my_shell"), config);
        assert_eq!(s.name, "my_shell");
    }

    #[test]
    fn can_resolve_its_path() {
        let root_path = root_path("/some/random/path");
        let config = mock_config(root_path.clone());
        let s = Shell::new("default", config);

        let expected_path = root_path.join("shells").join("default");
        assert_eq!(s.root_path(), expected_path);
    }

    #[test]
    fn resolves_empty_string_to_root() {
        let root_path = root_path("/Users/geoff/.config/hermit");
        let config = mock_config(root_path.clone());
        let s = Shell::new("default", config);

        let expected_path = root_path.join("shells").join("default");
        assert_eq!(s.path_for(""), expected_path);
    }

    #[test]
    fn can_resolve_paths() {
        let root_path = root_path("/Users/geoff/.config/hermit");
        let config = mock_config(root_path.clone());
        let s = Shell::new("default", config);

        let expected_path = root_path.join("shells").join("default").join(".bashrc");
        assert_eq!(s.path_for(".bashrc"), expected_path);
    }
}
