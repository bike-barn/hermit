use std::fs;
use std::io;
use std::path::{Path, PathBuf};

enum Op {
    MkDir(PathBuf),
    GitInit(PathBuf),
}

struct FileOperations {
    root: PathBuf,
    operations: Vec<Op>,
}

impl FileOperations {
    pub fn rooted_at<P: AsRef<Path>>(path: P) -> FileOperations {
        FileOperations {
            root: PathBuf::from(path.as_ref()),
            operations: vec![],
        }
    }

    pub fn make_dir<P: AsRef<Path>>(&mut self, name: P) {
        self.operations.push(Op::MkDir(self.root.join(name)))
    }

    pub fn make_git_repo<P: AsRef<Path>>(&mut self, name: P) {
        self.operations.push(Op::GitInit(self.root.join(name)))
    }

    pub fn commit(mut self) -> Vec<io::Result<()>> {
        let ops = self.operations;
        self.operations = vec![];
        self.operations.push(Op::MkDir(PathBuf::from(".")));

        ops.into_iter()
           .map(|op| {
               match op {
                   Op::MkDir(dir) => self.create_dir(dir),
                   Op::GitInit(dir) => self.create_git_repo(dir),
               }
           })
           .collect::<Vec<_>>()
    }

    /// Private Methods

    fn create_dir(&mut self, dir: PathBuf) -> io::Result<()> {
        fs::create_dir(dir)
    }

    fn create_git_repo(&mut self, dir: PathBuf) -> io::Result<()> {
        fs::create_dir(dir.join(".git"))
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use super::FileOperations;

    fn clean_up(test_root: &PathBuf) {
        if test_root.exists() {
            fs::remove_dir_all(test_root).unwrap();
        }
        assert!(!test_root.is_dir());
    }

    fn set_up(suffix: &str) -> PathBuf {
        let test_root = PathBuf::from("./target/file_set_tests-".to_owned() + suffix);
        clean_up(&test_root);
        fs::create_dir(&test_root).unwrap();
        assert!(test_root.is_dir());

        test_root
    }

    #[test]
    fn can_create_a_directory() {
        let test_root = set_up("mkdir");
        let mut file_set = FileOperations::rooted_at(&test_root);

        file_set.make_dir("test");
        assert!(!test_root.join("test").is_dir());
        file_set.commit();
        assert!(test_root.join("test").is_dir());

        clean_up(&test_root);
    }

    #[test]
    fn can_init_a_git_repo() {
        let test_root = set_up("git");
        let mut file_set = FileOperations::rooted_at(&test_root);

        file_set.make_git_repo(".");
        assert!(!test_root.join(".git").is_dir());
        file_set.commit();
        assert!(test_root.join(".git").is_dir());

        clean_up(&test_root);
    }
}
