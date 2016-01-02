use std::path::PathBuf;

pub trait Shell {
    fn to_path(&self) -> PathBuf;
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
    fn to_path(&self) -> PathBuf {
        self.root_path.join("shells").join(&self.name)
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
        let expected_path = root_path.join("shells/default");
        let s = ShellImpl::new("default", root_path);
        assert_eq!(s.to_path(), expected_path);
    }
}
