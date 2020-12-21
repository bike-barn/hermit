use crate::common::*;

pub struct Shell<T: Config> {
    pub name: String,
    pub config: Rc<T>,
}

impl<T: Config> Shell<T> {
    pub fn new(name: impl Into<String>, config: Rc<T>) -> Shell<T> {
        let name = name.into();
        Shell { name, config }
    }

    pub fn root_path(&self) -> PathBuf {
        self.config.shell_root_path().join(&self.name)
    }

    #[allow(dead_code)]
    pub fn path_for(&self, filename: &str) -> PathBuf {
        self.root_path().join(filename)
    }

    pub fn link(&self, file_operations: &mut FileOperations) {
        let shell_root = self.root_path();
        for path in self.config.shell_files(&self.name) {
            file_operations.link(&path, shell_root.join(&path))
        }
    }

    pub fn unlink(&self, file_operations: &mut FileOperations) {
        for path in self.config.shell_files(&self.name) {
            file_operations.remove(&path)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{
        path::{Path, PathBuf},
        rc::Rc,
    };

    use crate::{config::mock::MockConfig, file_operations::Op, test_helpers::ops::*};

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

    #[test]
    fn can_link_all_paths() {
        let root_path = root_path("/Users/geoff/.config/hermit");
        let mut config = MockConfig::with_root(&root_path);
        config.set_paths(vec![".bashrc", ".boot/profile.boot"]);
        let s = Shell::new("default", Rc::new(config));
        let op_root = PathBuf::from("op_root");
        let mut file_ops = FileOperations::rooted_at(&op_root);

        s.link(&mut file_ops);

        let shell_root = s.root_path();
        assert_eq!(
            file_ops.operations(),
            &vec![
                link_op_for(&shell_root, &op_root, ".bashrc"),
                link_op_for(&shell_root, &op_root, ".boot/profile.boot")
            ]
        );
    }

    #[test]
    fn can_unlink_all_paths() {
        let root_path = root_path("/Users/geoff/.config/hermit");
        let mut config = MockConfig::with_root(&root_path);
        config.set_paths(vec![".bashrc", ".boot/profile.boot"]);
        let s = Shell::new("default", Rc::new(config));
        let op_root = PathBuf::from("op_root");
        let mut file_ops = FileOperations::rooted_at(&op_root);

        s.unlink(&mut file_ops);

        assert_eq!(
            file_ops.operations(),
            &vec![
                Op::Remove(op_root.join(".bashrc")),
                Op::Remove(op_root.join(".boot/profile.boot"))
            ]
        );
    }
}
