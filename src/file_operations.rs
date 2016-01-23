use std::{error, fmt, fs, io, result};
use std::path::{Path, PathBuf};
use std::os;

use git2;

#[derive(PartialEq,Eq)]
#[derive(Debug)]
pub enum Op {
    MkDir(PathBuf),
    MkDirAll(PathBuf),
    GitInit(PathBuf),
    Link(PathBuf, PathBuf),
    Unlink(PathBuf),
}

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    Git2Error(git2::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::IoError(ref err) => write!(f, "IO error: {}", err),
            Error::Git2Error(ref err) => write!(f, "Git2 error: {}", err),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IoError(ref err) => err.description(),
            Error::Git2Error(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::IoError(ref err) => Some(err),
            Error::Git2Error(ref err) => Some(err),
        }
    }
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
    pub root: PathBuf,
    pub operations: Vec<Op>,
    git_init_opts: git2::RepositoryInitOptions,
}

impl FileOperations {
    pub fn rooted_at<P: AsRef<Path>>(path: P) -> FileOperations {
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

    pub fn create_dir<P: AsRef<Path>>(&mut self, name: P) {
        self.operations.push(Op::MkDir(self.root.join(name)))
    }

    pub fn create_dir_all<P: AsRef<Path>>(&mut self, name: P) {
        self.operations.push(Op::MkDirAll(self.root.join(name)))
    }

    pub fn link<P: AsRef<Path>>(&mut self, source: P, dest: P) {
        self.operations.push(Op::Link(self.root.join(source), self.root.join(dest)))
    }

    pub fn unlink<P: AsRef<Path>>(&mut self, name: P) {
        self.operations.push(Op::Unlink(self.root.join(name)))
    }

    pub fn create_git_repo<P: AsRef<Path>>(&mut self, name: P) {
        self.operations.push(Op::GitInit(self.root.join(name)))
    }

    pub fn commit(mut self) -> Vec<Result> {
        let ops = self.operations;
        self.operations = vec![];
        self.operations.push(Op::MkDir(PathBuf::new()));

        ops.into_iter()
           .map(|op| self.do_op(op))
           .collect::<Vec<_>>()
    }

    /// Private Methods

    fn do_op(&mut self, op: Op) -> Result {
        match op {
            Op::MkDir(dir) => try!(fs::create_dir(dir)),
            Op::MkDirAll(dir) => try!(fs::create_dir_all(dir)),
            Op::GitInit(dir) => try!(self.git_init(dir)),
            Op::Link(src, dst) => try!(os::unix::fs::symlink(src, dst)),
            Op::Unlink(file) => try!(fs::remove_file(file)),
        };
        Ok(())
    }

    fn git_init(&self, dir: PathBuf) -> result::Result<(), git2::Error> {
        git2::Repository::init_opts(dir, &self.git_init_opts).map(|_| ())
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::fs;

    use super::FileOperations;
    use test_helpers::filesystem::{set_up, clean_up};

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
    fn can_link_file() {
        let test_root = set_up("link");
        let mut file_set = FileOperations::rooted_at(&test_root);

        let mut f = fs::File::create(test_root.join("file_a")).unwrap();
        
        file_set.link("file_a", "file_b");
        let results = file_set.commit();
        assert_eq!(results.len(), 1);
        assert!(results[0].is_ok());

        let attr = fs::symlink_metadata(test_root.join("file_b")).unwrap();
        assert!(attr.file_type().is_symlink());

        clean_up(&test_root);
    }

    #[test]
    fn can_unlink_file() {
        let test_root = set_up("unlink");
        let mut file_set = FileOperations::rooted_at(&test_root);

        // Create symbolic link to remove
        let mut f = fs::File::create(test_root.join("file_a")).unwrap();
        file_set.link("file_a", "file_b");
        file_set.unlink("file_b");
        let results = file_set.commit();

        assert_eq!(results.len(), 2);
        assert!(results[1].is_ok());
        assert!(!test_root.join("file_b").exists());

        clean_up(&test_root);
    }

    #[test]
    fn cannot_create_a_directory_in_a_nonexistent_path() {
        let test_root = set_up("not-mkdir");
        let mut file_set = FileOperations::rooted_at(&test_root);

        assert!(!test_root.join("test").is_dir());
        let path = Path::new("test").join("one").join("two").join("three");
        file_set.create_dir(path);
        assert!(!test_root.join("test").is_dir());
        let results = file_set.commit();
        assert_eq!(results.len(), 1);
        assert!(results[0].is_err());
        assert!(!test_root.join("test").is_dir());

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

    #[test]
    fn can_init_a_git_repo_at_a_nonexistent_path() {
        let test_root = set_up("git-deep");
        let mut file_set = FileOperations::rooted_at(&test_root);

        let path = Path::new("test").join("sub").join("repo");
        assert!(!test_root.join(&path).join(".git").is_dir());
        file_set.create_git_repo(&path);
        assert!(!test_root.join(&path).join(".git").is_dir());
        let results = file_set.commit();
        assert_eq!(results.len(), 1);
        assert!(results[0].is_ok());
        assert!(test_root.join(&path).join(".git").is_dir());

        clean_up(&test_root);
    }

    #[test]
    fn wont_re_init_an_already_existing_repository() {
        let test_root = set_up("git-re-init");
        let mut file_set = FileOperations::rooted_at(&test_root);

        file_set.create_git_repo(".");
        file_set.create_git_repo(".");

        let results = file_set.commit();
        assert_eq!(results.len(), 2);
        assert!(results[0].is_ok());
        assert!(results[1].is_err());
    }
}
