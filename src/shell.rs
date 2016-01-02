use std::path::PathBuf;

pub trait Shell {
    fn root_path(&self) -> PathBuf;

    fn path_for(&self, filename: &str) -> PathBuf;
}

pub struct ShellImpl {
    name: String,
    root_path: PathBuf,
}

impl ShellImpl {
    pub fn new(name: &str, root_path: PathBuf) -> ShellImpl {
        ShellImpl {
            name: name.to_string(),
            root_path: root_path,
        }
    }
}

impl Shell for ShellImpl {
    fn root_path(&self) -> PathBuf {
        self.root_path.join("shells").join(&self.name)
    }

    fn path_for(&self, filename: &str) -> PathBuf {
        self.root_path().join(filename)
    }
}

#[cfg(test)]
mod tests {

    use std::path::PathBuf;
    use super::{Shell, ShellImpl};

    fn root_path(path_str: &str) -> PathBuf {
        PathBuf::from(path_str)
    }

    #[test]
    fn has_a_name() {
        let s = ShellImpl::new("my_shell", root_path("/"));
        assert_eq!(s.name, "my_shell");
    }

    #[test]
    fn can_resolve_its_path() {
        let root_path = root_path("/Users/geoff/.config/hermit");
        let expected_path = root_path.join("shells")
                                     .join("default");
        let s = ShellImpl::new("default", root_path);

        assert_eq!(s.root_path(), expected_path);
    }

    #[test]
    fn resolves_empty_string_to_root() {
        let root_path = root_path("/Users/geoff/.config/hermit");
        let expected_path = root_path.join("shells")
                                     .join("default");
        let s = ShellImpl::new("default", root_path);

        assert_eq!(s.path_for(""), expected_path);
    }

    #[test]
    fn can_resolve_paths() {
        let root_path = root_path("/Users/geoff/.config/hermit");
        let expected_path = root_path.join("shells")
                                     .join("default")
                                     .join(".bashrc");
        let s = ShellImpl::new("default", root_path);

        assert_eq!(s.path_for(".bashrc"), expected_path);
    }
}
