use std::path::PathBuf;

pub struct Shell<'a> {
    pub name: String,
    pub root_path: &'a PathBuf,
}

impl<'a> Shell<'a> {
    pub fn new<S>(name: S, root_path: &'a PathBuf) -> Shell<'a>
        where S: Into<String>
    {
        Shell {
            name: name.into(),
            root_path: root_path,
        }
    }

    pub fn root_path(&self) -> PathBuf {
        self.root_path.join("shells").join(&self.name)
    }

    pub fn path_for(&self, filename: &str) -> PathBuf {
        self.root_path().join(filename)
    }
}

#[cfg(test)]
mod tests {

    use std::path::PathBuf;
    use super::Shell;

    fn root_path(path_str: &str) -> PathBuf {
        PathBuf::from(path_str)
    }

    #[test]
    fn has_a_name() {
        let root_path = root_path("/");
        let s = Shell::new("my_shell", &root_path);
        assert_eq!(s.name, "my_shell");
    }

    #[test]
    fn has_a_string_name() {
        let root_path = root_path("/");
        let s = Shell::new(String::from("my_shell"), &root_path);
        assert_eq!(s.name, "my_shell");
    }

    #[test]
    fn can_resolve_its_path() {
        let root_path = root_path("/Users/geoff/.config/hermit");
        let s = Shell::new("default", &root_path);

        let expected_path = root_path.join("shells")
                                     .join("default");
        assert_eq!(s.root_path(), expected_path);
    }

    #[test]
    fn resolves_empty_string_to_root() {
        let root_path = root_path("/Users/geoff/.config/hermit");
        let s = Shell::new("default", &root_path);

        let expected_path = root_path.join("shells")
                                     .join("default");
        assert_eq!(s.path_for(""), expected_path);
    }

    #[test]
    fn can_resolve_paths() {
        let root_path = root_path("/Users/geoff/.config/hermit");
        let s = Shell::new("default", &root_path);

        let expected_path = root_path.join("shells")
                                     .join("default")
                                     .join(".bashrc");
        assert_eq!(s.path_for(".bashrc"), expected_path);
    }
}
