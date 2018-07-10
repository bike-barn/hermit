use std::{fs, io, mem, result};
use std::os::unix;
use std::path::{Path, PathBuf};

use git2;

#[derive(Debug, PartialEq, Eq)]
pub enum Op {
    MkDir(PathBuf),
    GitInit(PathBuf),
    Link { path: PathBuf, target: PathBuf },
    Remove(PathBuf),
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "IO error: {}", _0)]
    IoError(#[cause] io::Error),

    #[fail(display = "Git2 error: {}", _0)]
    Git2Error(#[cause] git2::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}

impl From<git2::Error> for Error {
    fn from(err: git2::Error) -> Error {
        Error::Git2Error(err)
    }
}

pub type Result = result::Result<(), Error>;

pub struct FileOperations {
    root: PathBuf,
    operations: Vec<Op>,
    git_init_opts: git2::RepositoryInitOptions,
}

impl FileOperations {
    pub fn rooted_at(path: impl AsRef<Path>) -> FileOperations {
        FileOperations {
            root: PathBuf::from(path.as_ref()),
            operations: vec![],
            git_init_opts: FileOperations::default_git_opts(),
        }
    }

    fn default_git_opts() -> git2::RepositoryInitOptions {
        let mut opts = git2::RepositoryInitOptions::new();
        opts.no_reinit(true);

        opts
    }

    #[allow(dead_code)]
    pub fn operations(&self) -> &Vec<Op> {
        &self.operations
    }

    #[allow(dead_code)]
    pub fn create_dir(&mut self, name: impl AsRef<Path>) {
        self.operations.push(Op::MkDir(self.root.join(name)))
    }

    pub fn link(&mut self, path: impl AsRef<Path>, target: impl AsRef<Path>) {
        self.operations.push(Op::Link{
            path: self.root.join(path),
            target: target.as_ref().to_path_buf(),
        });
    }

    pub fn remove(&mut self, file: impl AsRef<Path>) {
        self.operations.push(Op::Remove(self.root.join(file)));
    }

    pub fn create_git_repo(&mut self, name: impl AsRef<Path>) {
        self.operations.push(Op::GitInit(self.root.join(name)))
    }

    pub fn commit(mut self) -> Vec<Result> {
        mem::replace(&mut self.operations, vec![]).into_iter()
            .map(|op| self.do_op(op))
            .collect::<Vec<_>>()
    }

    /// Private Methods

    fn do_op(&mut self, op: Op) -> Result {
        match op {
            Op::MkDir(dir) => fs::create_dir_all(dir)?,
            Op::GitInit(dir) => git_init(dir, &self.git_init_opts)?,
            Op::Link { path, target } => unix::fs::symlink(target, path)?,
            Op::Remove(file) => fs::remove_file(file)?,
        };
        Ok(())
    }
}

fn git_init(dir: PathBuf, options: &git2::RepositoryInitOptions) -> result::Result<(), git2::Error> {
    git2::Repository::init_opts(dir, options).map(|_| ())
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};
    use std::fs;

    use super::FileOperations;
    use test_helpers::filesystem::set_up;

    #[test]
    fn can_link_file() {
        let test_root = set_up("link");
        let target_root = set_up("link-target");

        let mut file_set = FileOperations::rooted_at(&test_root);
        let target_path = target_root.join("target_file");
        let link_path = test_root.join("link");

        fs::File::create(&target_path).unwrap();

        file_set.link("link", &target_path);
        let results = file_set.commit();

        assert_eq!(results.len(), 1);
        results[0].as_ref().expect("Op failed");

        match fs::symlink_metadata(&link_path) {
            Ok(val) => assert!(val.file_type().is_symlink()),
            Err(_err) => panic!("{:?} does not exist", link_path),
        };
    }

    #[test]
    fn does_not_link_file_without_commit() {
        let test_root = PathBuf::from("no-link");
        let mut file_set = FileOperations::rooted_at(&test_root);
        let target_path = test_root.join("target_file");
        let link_path = test_root.join("link");

        assert!(! link_path.exists());
        file_set.link("link", &target_path);
        assert!(! link_path.exists());
    }

    #[test]
    fn can_remove_file() {
        let test_root = set_up("unlink");
        let mut file_set = FileOperations::rooted_at(&test_root);

        // Create file to remove
        fs::File::create(test_root.join("file_a")).unwrap();
        file_set.remove("file_a");
        let results = file_set.commit();

        assert_eq!(results.len(), 1);
        results[0].as_ref().expect("Op failed");
        assert!(!test_root.join("file_a").exists());
    }

    #[test]
    fn does_not_remove_file_without_commit() {
        let test_root = set_up("no-unlink");
        let mut file_set = FileOperations::rooted_at(&test_root);
        let file_path = test_root.join("file_a");
        // Create file to remove
        fs::File::create(&file_path).unwrap();

        assert!(file_path.exists());
        file_set.remove("file_a");
        assert!(file_path.exists());
    }

    #[test]
    fn can_create_a_directory() {
        let test_root = set_up("mkdir");
        let mut file_set = FileOperations::rooted_at(&test_root);

        assert!(!test_root.join("test").is_dir());
        file_set.create_dir("test");

        let results = file_set.commit();
        assert_eq!(results.len(), 1);
        results[0].as_ref().expect("Op failed");
        assert!(test_root.join("test").is_dir());
    }

    #[test]
    fn does_not_create_a_directory_without_commit() {
        let test_root = set_up("no-mkdir");
        let mut file_set = FileOperations::rooted_at(&test_root);

        assert!(!test_root.join("test").is_dir());
        file_set.create_dir("test");
        assert!(!test_root.join("test").is_dir());
    }

    #[test]
    fn can_create_path_of_needed_directories() {
        let test_root = set_up("mkdir-deep");
        let mut file_set = FileOperations::rooted_at(&test_root);

        let path = Path::new("test").join("one").join("two").join("three");
        file_set.create_dir(path);

        let results = file_set.commit();
        assert_eq!(results.len(), 1);
        results[0].as_ref().expect("Op failed");
        assert!(test_root.join("test").is_dir());
    }

    #[test]
    fn can_init_a_git_repo() {
        let test_root = set_up("git-init");
        let mut file_set = FileOperations::rooted_at(&test_root);

        file_set.create_git_repo(".");

        let results = file_set.commit();
        assert_eq!(results.len(), 1);
        results[0].as_ref().expect("Op failed");
        assert!(test_root.join(".git").is_dir());
    }

    #[test]
    fn does_not_init_without_commit() {
        let test_root = set_up("no-git-init");
        let mut file_set = FileOperations::rooted_at(&test_root);
        let path = Path::new("test").join("repo");
        let git_dir_path = path.join(".git");

        assert!(! git_dir_path.is_dir());
        file_set.create_git_repo(&path);
        assert!(! git_dir_path.is_dir());
    }

    #[test]
    fn can_init_a_git_repo_at_a_nonexistent_path() {
        let test_root = set_up("git-deep");
        let mut file_set = FileOperations::rooted_at(&test_root);
        let path = Path::new("test").join("sub").join("repo");

        file_set.create_git_repo(&path);

        let results = file_set.commit();
        assert_eq!(results.len(), 1);
        results[0].as_ref().expect("Op failed");
        assert!(test_root.join(&path).join(".git").is_dir());
    }

    #[test]
    fn wont_re_init_an_already_existing_repository() {
        let test_root = set_up("git-re-init");
        let mut file_set = FileOperations::rooted_at(&test_root);

        file_set.create_git_repo(".");
        file_set.create_git_repo(".");

        let results = file_set.commit();
        assert_eq!(results.len(), 2);
        results[0].as_ref().expect("Op failed");
        results[1].as_ref().expect_err("Op unexpectedly succeeded");
    }
}
