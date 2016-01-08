use std::{fs, io, result};
use std::path::{Path, PathBuf};

use git2;

enum Op {
    MkDir(PathBuf),
    MkDirAll(PathBuf),
    GitInit(PathBuf),
}

enum Error {
    IoError(io::Error),
    Git2Error(git2::Error),
}

type Result = result::Result<(), Error>;

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

    pub fn create_dir<P: AsRef<Path>>(&mut self, name: P) {
        self.operations.push(Op::MkDir(self.root.join(name)))
    }

    pub fn create_dir_all<P: AsRef<Path>>(&mut self, name: P) {
        self.operations.push(Op::MkDirAll(self.root.join(name)))
    }

    pub fn create_git_repo<P: AsRef<Path>>(&mut self, name: P) {
        self.operations.push(Op::GitInit(self.root.join(name)))
    }

    pub fn commit(mut self) -> Vec<Result> {
        let ops = self.operations;
        self.operations = vec![];
        self.operations.push(Op::MkDir(PathBuf::new()));

        ops.into_iter()
           .map(|op| {
               match op {
                   Op::MkDir(dir) => self.make_dir(dir),
                   Op::MkDirAll(dir) => self.make_dir_all(dir),
                   Op::GitInit(dir) => self.make_git_repo(dir),
               }
           })
           .collect::<Vec<_>>()
    }

    /// Private Methods

    fn make_dir(&mut self, dir: PathBuf) -> Result {
        fs::create_dir(dir).map_err(|e| Error::IoError(e))
    }

    fn make_dir_all(&mut self, dir: PathBuf) -> Result {
        fs::create_dir_all(dir).map_err(|e| Error::IoError(e))
    }

    fn make_git_repo(&mut self, dir: PathBuf) -> Result {
        git2::Repository::init(dir).map(|_| ()).map_err(|e| Error::Git2Error(e))
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::{Path, PathBuf};

    use super::FileOperations;

    fn clean_up(test_root: &PathBuf) {
        if test_root.exists() && test_root.is_dir() {
            fs::remove_dir_all(test_root).expect("Recursive dir removal to succeed.");
        }
        assert!(!test_root.exists());
    }

    fn set_up(suffix: &str) -> PathBuf {
        let test_root = PathBuf::from("./target/file_set_tests".to_owned()).join(suffix);
        clean_up(&test_root);
        fs::create_dir_all(&test_root)
            .expect(format!("expected dir not to exist: {}", suffix).as_ref());
        assert!(test_root.exists());
        assert!(test_root.is_dir());

        test_root
    }

    #[test]
    fn can_create_a_directory() {
        let test_root = set_up("mkdir");
        let mut file_set = FileOperations::rooted_at(&test_root);

        assert!(!test_root.join("test").is_dir());
        file_set.create_dir("test");
        assert!(!test_root.join("test").is_dir());
        let results = file_set.commit();
        assert_eq!(results.len(), 1);
        assert!(results[0].is_ok());
        assert!(test_root.join("test").is_dir());

        clean_up(&test_root);
    }

    #[test]
    fn can_create_path_of_needed_directories() {
        let test_root = set_up("mkdir-deep");
        let mut file_set = FileOperations::rooted_at(&test_root);

        assert!(!test_root.join("test").is_dir());
        let path = Path::new("test").join("one").join("two").join("three");
        file_set.create_dir_all(path);
        assert!(!test_root.join("test").is_dir());
        let results = file_set.commit();
        assert_eq!(results.len(), 1);
        assert!(results[0].is_ok());
        assert!(test_root.join("test").is_dir());

        clean_up(&test_root);
    }

    #[test]
    fn can_init_a_git_repo() {
        let test_root = set_up("git");
        let mut file_set = FileOperations::rooted_at(&test_root);

        assert!(!test_root.join(".git").is_dir());
        file_set.create_git_repo(".");
        assert!(!test_root.join(".git").is_dir());
        let results = file_set.commit();
        assert_eq!(results.len(), 1);
        assert!(results[0].is_ok());
        assert!(test_root.join(".git").is_dir());

        clean_up(&test_root);
    }
}
