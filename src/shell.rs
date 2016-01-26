extern crate walkdir;

use std::path::PathBuf;
use std::fs;
use std::env;
use file_operations::FileOperations;


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

    pub fn create_links(&self) -> FileOperations {
        let mut file_ops = FileOperations::rooted_at(env::home_dir().unwrap());
        let entries = fs::read_dir(self.root_path()).unwrap();
        let shell_path_length = self.root_path().to_str().unwrap().len();
        let file_root_length = file_ops.root.to_str().unwrap().len();
        for entry in entries {
            let entry = entry.unwrap().path();
            let entry = entry.to_str().unwrap();
            let rel_shell = &entry[shell_path_length+1..entry.len()];
            let rel_file_root = &entry[file_root_length+1..entry.len()];
            file_ops.link(rel_file_root, rel_shell);
        }
        return file_ops;
    }
}

#[cfg(test)]
mod tests {

    use std::path::PathBuf;
    use super::Shell;
    use std::env;
    use file_operations::FileOperations;
    use std::fs;

    use test_helpers::filesystem::{set_up, clean_up};

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

    #[test]
    fn can_link_files() {
        let test_root = set_up("shell");
    
        // save original home directory
        let og_home = env::home_dir();
        env::set_var("HOME", &test_root);
        fs::create_dir(test_root.join(".config"));
        fs::create_dir(test_root.join(".config/hermit"));
        fs::create_dir(test_root.join(".config/hermit/shells"));
        fs::create_dir(test_root.join(".config/hermit/shells/default"));

        let shell_root = test_root.join(".config/hermit/shells/default");
        fs::File::create(shell_root.join(".bashrc"));
        fs::File::create(shell_root.join(".gitconfig"));
        fs::create_dir(shell_root.join("foo"));
        fs::File::create(shell_root.join("foo/bar.txt"));

        let hermit_root = test_root.join(".config/hermit");
        let s = Shell::new("default", &hermit_root);
        let file_ops = s.create_links();
        let results = file_ops.commit();
        assert_eq!(results.len(), 3);
        assert!(results[0].is_ok());
        assert!(results[1].is_ok());
        assert!(results[2].is_ok());
        let rel_shell = PathBuf::from(".config/hermit/shells/default");
        let link_addr = fs::read_link(test_root.join(".bashrc")).unwrap();
        println!("First");
        assert_eq!(link_addr, rel_shell.join(".bashrc"));
        let link_addr = fs::read_link(test_root.join(".gitconfig")).unwrap();
        println!("Second");
        assert_eq!(link_addr, rel_shell.join(".gitconfig"));
        let link_addr = fs::read_link(test_root.join("foo")).unwrap();
        println!("Third");
        assert_eq!(link_addr, rel_shell.join("foo"));
        assert!(test_root.join("foo/bar.txt").exists());
        clean_up(&test_root);
    }
}
