use std::fs;
use std::io;
use std::path::{Path, PathBuf};

enum FileOp {
    MkDir(PathBuf),
    GitInit(PathBuf),
}

struct FileSet {
    root: PathBuf,
    operations: Vec<FileOp>,
}

impl FileSet {
    fn rooted_at<P: AsRef<Path>>(path: P) -> FileSet {
        FileSet {
            root: PathBuf::from(path.as_ref()),
            operations: vec![],
        }
    }

    fn make_dir<P: AsRef<Path>>(&mut self, name: P) {
        let path = PathBuf::from(name.as_ref());
        self.operations.push(FileOp::MkDir(path))
    }

    fn make_git_repo<P: AsRef<Path>>(&mut self, name: P) {
        let path = PathBuf::from(name.as_ref());
        self.operations.push(FileOp::GitInit(path))
    }

    fn commit(mut self) -> Vec<io::Result<()>> {
        let ops = self.operations;
        self.operations = vec![];
        self.operations.push(FileOp::MkDir(PathBuf::from(".")));

        ops.into_iter()
           .map(|op| {
               match op {
                   FileOp::MkDir(ref dir) => fs::create_dir(self.root.join(dir)),
                   FileOp::GitInit(ref dir) => fs::create_dir(self.root.join(dir).join(".git")),
               }
           })
           .collect::<Vec<_>>()
    }
}


#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use super::FileSet;

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
        let mut file_set = FileSet::rooted_at(&test_root);

        file_set.make_dir("test");
        assert!(!test_root.join("test").is_dir());
        file_set.commit();
        assert!(test_root.join("test").is_dir());

        clean_up(&test_root);
    }

    #[test]
    fn can_init_a_git_repo() {
        let test_root = set_up("git");
        let mut file_set = FileSet::rooted_at(&test_root);

        file_set.make_git_repo(".");
        assert!(!test_root.join(".git").is_dir());
        file_set.commit();
        assert!(test_root.join(".git").is_dir());

        clean_up(&test_root);
    }
}
