use std::fs;
use std::path::{Path, PathBuf};

struct FileSet {
    root: PathBuf,
    dir_names: Vec<PathBuf>,
    git_names: Vec<PathBuf>,
}

impl FileSet {
    fn rooted_at<P: AsRef<Path>>(path: P) -> FileSet {
        FileSet {
            root: PathBuf::from(path.as_ref()),
            dir_names: vec![],
            git_names: vec![],
        }
    }

    fn make_dir<P: AsRef<Path>>(&mut self, name: P) {
        self.dir_names.push(PathBuf::from(name.as_ref()))
    }

    fn make_git_repo<P: AsRef<Path>>(&mut self, name: P) {
        self.git_names.push(PathBuf::from(name.as_ref()))
    }

    fn commit(&mut self) {
        for dir in &self.dir_names {
            fs::create_dir(self.root.join(dir)).unwrap()
        }
        for dir in &self.git_names {
            fs::create_dir(self.root.join(dir).join(".git")).unwrap()
        }
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
