extern crate walkdir;

use std::path::{Path, PathBuf};
use std::{fs, env, io};
use file_operations::FileOperations;
use std::string::ParseError;

#[derive(Debug)]
enum ShellError {
    NoRootPath(io::Error),
}

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

    pub fn create_links<P: AsRef<Path>>(&self, base_directory: P) -> Result<FileOperations, ShellError> {
        let mut file_ops = FileOperations::rooted_at(base_directory);
        let entries = try!(fs::read_dir(self.root_path()));
        //let entries = fs::read_dir(self.root_path()).unwrap();
        //let shell_path_length = self.root_path().to_str().unwrap().len();
        //let shell_path_length = try!(self.root_path().to_str()).len();
        let shell_path_length = try!(self.root_path().to_str().ok_or(NoRootPath));
        /*
        let file_root_length = file_ops.root.to_str().unwrap().len();
        for entry in entries {
            let entry = entry.unwrap().path();
            let entry = entry.to_str().unwrap();
            let rel_shell = &entry[shell_path_length+1..entry.len()];
            let rel_file_root = &entry[file_root_length+1..entry.len()];
            file_ops.link(rel_file_root, rel_shell);
        }
        */
        return Ok(file_ops);
    }

    pub fn remove_links<P: AsRef<Path>>(&self, base_directory: P) -> FileOperations {
        let mut file_ops = FileOperations::rooted_at(&base_directory);
        for entry in walkdir::WalkDir::new(&base_directory) {
            let entry = entry.unwrap();
            match fs::read_link(entry.path()) {
                Ok(i) => {
                    let fname = entry.path().file_name().unwrap().to_str().unwrap();
                    let link_path = base_directory.as_ref().join(&i);
                    let shell_path = self.path_for(fname);
                    if link_path == shell_path {
                        let entry_str = entry.path().to_str().unwrap();
                        let base_str = base_directory.as_ref().to_str().unwrap();
                        let rel_to_root = &entry_str[base_str.len()+1..entry_str.len()];
                        file_ops.remove(rel_to_root);
                    }
                },
                Err(i) => continue,
            }
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
    use std::os;
    use std::thread;
    use std::time::Duration;

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
        /*let test_root = set_up("shell-link");
    
        fs::create_dir(test_root.join(".config"));
        fs::create_dir(test_root.join(".config/hermit"));
        fs::create_dir(test_root.join(".config/hermit/shells"));
        fs::create_dir(test_root.join(".config/hermit/shells/default"));

        let shell_root = test_root.join(".config/hermit/shells/default");
        fs::File::create(shell_root.join(".bashrc"));
        fs::File::create(shell_root.join(".gitconfig"));
        fs::create_dir(shell_root.join("foo"));
        fs::File::create(shell_root.join("foo/bar.txt"));
        */

        let (test_root, shell_root) = setup_shell_files("shell_link");

        let hermit_root = test_root.join(".config/hermit");
        let s = Shell::new("default", &hermit_root);
        let file_ops = s.create_links(&test_root).unwrap();
        let results = file_ops.commit();
        assert_eq!(results.len(), 3);
        assert!(results[0].is_ok());
        assert!(results[1].is_ok());
        assert!(results[2].is_ok());
        let rel_shell = PathBuf::from(".config/hermit/shells/default");
        let link_addr = fs::read_link(test_root.join(".bashrc")).unwrap();
        assert_eq!(link_addr, rel_shell.join(".bashrc"));
        let link_addr = fs::read_link(test_root.join(".gitconfig")).unwrap();
        assert_eq!(link_addr, rel_shell.join(".gitconfig"));
        let link_addr = fs::read_link(test_root.join("foo")).unwrap();
        assert_eq!(link_addr, rel_shell.join("foo"));
        assert!(test_root.join("foo/bar.txt").exists());

        clean_up(&test_root);
    }

    #[test]
    fn can_remove_links() {
        /*let test_root =  set_up("shell-unlink");

        // set up shell directory
        fs::create_dir(test_root.join(".config"));
        fs::create_dir(test_root.join(".config/hermit"));
        fs::create_dir(test_root.join(".config/hermit/shells"));
        fs::create_dir(test_root.join(".config/hermit/shells/default"));

        let shell_root = test_root.join(".config/hermit/shells/default");
        fs::File::create(shell_root.join(".bashrc"));
        fs::File::create(shell_root.join(".gitconfig"));
        fs::create_dir(shell_root.join("foo"));
        fs::File::create(shell_root.join("foo/bar.txt"));
        */

        let (test_root, shell_root) = setup_shell_files("shell_unlink");

        // set up symlinks
        os::unix::fs::symlink(".config/hermit/shells/default/.bashrc", test_root.join(".bashrc"));
        os::unix::fs::symlink(".config/hermit/shells/default/.gitconfig", test_root.join(".gitconfig"));
        os::unix::fs::symlink(".config/hermit/shells/default/foo", test_root.join("foo"));

        let hermit_root = test_root.join(".config/hermit");
        let s = Shell::new("default", &hermit_root);
        let file_ops = s.remove_links(&test_root);
        let results = file_ops.commit();

        assert_eq!(results.len(), 3);
        assert!(results[0].is_ok());
        assert!(results[1].is_ok());
        assert!(results[2].is_ok());

        assert!(!test_root.join(".bashrc").exists());
        assert!(!test_root.join(".gitconfig").exists());
        assert!(!test_root.join("foo").exists());
        clean_up(&test_root);
    }

    fn setup_shell_files(root: &str) -> (PathBuf, PathBuf) {
        let test_root =  set_up(root);

        // set up shell directory
        fs::create_dir(test_root.join(".config"));
        fs::create_dir(test_root.join(".config/hermit"));
        fs::create_dir(test_root.join(".config/hermit/shells"));
        fs::create_dir(test_root.join(".config/hermit/shells/default"));

        let shell_root = test_root.join(".config/hermit/shells/default");
        fs::File::create(shell_root.join(".bashrc"));
        fs::File::create(shell_root.join(".gitconfig"));
        fs::create_dir(shell_root.join("foo"));
        fs::File::create(shell_root.join("foo/bar.txt"));
        return (test_root, shell_root); 
    }
}
