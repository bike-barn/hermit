use std::borrow::Borrow;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::{fs, io};

use walkdir::{self, WalkDir};

pub trait Config {
    type IntoIterator: IntoIterator<Item = PathBuf>;

    fn root_path(&self) -> &PathBuf;

    fn shell_root_path(&self) -> PathBuf {
        self.root_path().join("shells")
    }

    fn current_shell_name(&self) -> Option<&str>;

    fn current_shell_path(&self) -> Option<PathBuf> {
        self.current_shell_name()
            .map(|name| self.shell_root_path().join(name))
    }

    fn set_current_shell_name(&mut self, name: &str) -> io::Result<()>;

    fn shell_exists(&self, name: &str) -> bool;

    fn shell_files(&self, name: &str) -> Self::IntoIterator;
}

#[derive(Clone)]
pub struct FsConfig {
    root_path: PathBuf,
    current_shell: Option<String>,
}

fn read_shell_from_path(path: &PathBuf) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut current_shell = String::new();

    file.read_to_string(&mut current_shell)?;

    Ok(current_shell)
}

fn config_path(root_path: &PathBuf) -> PathBuf {
    root_path.join("current_shell")
}

impl FsConfig {
    pub fn new(root_path: impl AsRef<Path>) -> FsConfig {
        let root_path = PathBuf::from(root_path.as_ref());
        fs::create_dir_all(&root_path);
        let config_path = config_path(&root_path);
        let current_shell = read_shell_from_path(&config_path).ok();

        FsConfig {
            root_path,
            current_shell,
        }
    }

    fn config_path(&self) -> PathBuf {
        config_path(&self.root_path())
    }
}

impl Config for FsConfig {
    type IntoIterator = Files;

    fn root_path(&self) -> &PathBuf {
        &self.root_path
    }

    fn current_shell_name(&self) -> Option<&str> {
        self.current_shell.as_ref().map(Borrow::borrow)
    }

    fn set_current_shell_name(&mut self, name: &str) -> io::Result<()> {
        let mut file = File::create(&self.config_path())?;

        file.write_all(name.as_bytes())?;

        self.current_shell = Some(name.to_string());

        Ok(())
    }

    fn shell_exists(&self, name: &str) -> bool {
        let shell_path = self.shell_root_path().join(name);
        shell_path.is_dir()
    }

    fn shell_files(&self, _name: &str) -> Self::IntoIterator {
        Files::new(self.current_shell_path())
    }
}

/// A wrapper on a DirEntry iterator.
///
/// This type can only be constructed by the `Files` wrapper, and it
/// handles cleaning up the iterator of `DirEntry`s into an iterator
/// of `PathBuf` to the files in that stream, and stripping them of
/// the walk root path prefix.
pub struct FilesIter<T>(Option<(T, PathBuf)>);

impl<T> Iterator for FilesIter<T>
where
    T: Iterator<Item = Result<walkdir::DirEntry, walkdir::Error>>,
{
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((ref mut iter, ref prefix_path)) = self.0 {
            loop {
                match iter.next() {
                    Some(Ok(entry)) => {
                        let file_type = entry.file_type();
                        if file_type.is_dir() {
                            continue;
                        }

                        let file_path = entry.path().to_path_buf();
                        let shell_relative_path = file_path
                            .strip_prefix(prefix_path)
                            .unwrap() // this unwrap is safe because
                            .to_path_buf(); // of the Files::new constructor
                        return Some(shell_relative_path);
                    }
                    Some(Err(_)) => continue,
                    None => return None,
                };
            }
        } else {
            None
        }
    }
}

/// A wrapper on WalkDir that handles nullability and bundles the walk
/// root path.
///
/// In particular, this pair of values is used to generate `PathBuf`s
/// relative to the specified root directory with
/// `PathBuf::strip_prefix`, and since the `WalkDir` was created with
/// the same path as `FilesIter` will use to strip the prefix, it is
/// always safe to just unwrap the result returned by `strip_prefix`.
pub struct Files(Option<(WalkDir, PathBuf)>);

impl Files {
    /// Constructs a new `Files` from a directory path.
    pub fn new(shell_path: Option<impl AsRef<Path>>) -> Files {
        let walker = shell_path.map(|path| {
            (
                WalkDir::new(&path).min_depth(1).follow_links(false),
                PathBuf::from(path.as_ref()),
            )
        });
        Files(walker)
    }
}

impl IntoIterator for Files {
    type Item = PathBuf;
    type IntoIter = FilesIter<walkdir::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        let Files(opt) = self;
        let iter_opt = opt.map(|(walker, path)| (walker.into_iter(), path));
        FilesIter(iter_opt)
    }
}

#[cfg(test)]
pub mod mock {
    use super::Config;

    use std::borrow::Borrow;
    use std::io;
    use std::path::{Path, PathBuf};

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct MockConfig {
        root_path: PathBuf,
        current_shell: String,
        allowed_shell_names: Vec<String>,
        files: Vec<PathBuf>,
    }

    impl MockConfig {
        pub fn new() -> MockConfig {
            MockConfig {
                root_path: PathBuf::from("/"),
                allowed_shell_names: vec!["default".to_owned()],
                current_shell: "default".to_owned(),
                files: vec![],
            }
        }

        pub fn with_root(root: impl AsRef<Path>) -> MockConfig {
            MockConfig {
                root_path: PathBuf::from(root.as_ref()),
                allowed_shell_names: vec!["default".to_owned()],
                current_shell: "default".to_owned(),
                files: vec![],
            }
        }

        pub fn set_paths(&mut self, paths: Vec<impl AsRef<Path>>) {
            self.files = paths
                .into_iter()
                .map(|p| PathBuf::from(p.as_ref()))
                .collect();
        }
    }

    impl Config for MockConfig {
        type IntoIterator = Vec<PathBuf>;

        fn root_path(&self) -> &PathBuf {
            &self.root_path
        }

        fn current_shell_name(&self) -> Option<&str> {
            Some(&self.current_shell).map(|shell_name| shell_name.borrow())
        }

        fn set_current_shell_name(&mut self, name: &str) -> io::Result<()> {
            self.current_shell = name.to_owned();
            Ok(())
        }

        fn shell_exists(&self, name: &str) -> bool {
            self.allowed_shell_names.contains(&name.to_owned())
        }

        fn shell_files(&self, _name: &str) -> Self::IntoIterator {
            self.files.clone()
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Config, FsConfig};

    use std::fs::{self, File};
    use std::io::prelude::*;
    use std::path::{Path, PathBuf};

    use tempfile::{tempdir, TempDir};

    fn set_up(suffix: &str, current: &str, shells: Vec<&str>) -> TempDir {
        let test_root_dir = tempdir().expect("failed to create tempdir");
        let test_root = test_root_dir.path();

        let path = test_root.join("current_shell");
        let mut file = File::create(&path).unwrap();
        file.write_all(current.as_bytes()).unwrap();

        let shell_root = test_root.join("shells");
        fs::create_dir(&shell_root).unwrap();
        for shell in shells {
            let new_shell = shell_root.join(PathBuf::from(shell));
            fs::create_dir(&new_shell).unwrap();
        }

        test_root_dir
    }

    #[test]
    fn has_a_root_path() {
        let test_root_dir = set_up("root-path", "default", vec!["default"]);
        let test_root = test_root_dir.path();
        let config = FsConfig::new(&test_root);
        assert_eq!(config.root_path(), &test_root);
    }

    #[test]
    fn creating_a_config_creates_its_root_dir() {
        let test_root_dir = set_up("root-path-create", "default", vec!["default"]);
        let test_root = test_root_dir.path();
        let config_root = test_root.join(".hermit");

        assert!(!config_root.exists());
        FsConfig::new(&config_root);
        assert!(config_root.exists());
    }

    #[test]
    fn returns_the_current_shell_name() {
        let test_root = set_up("current-shell-name", "current", vec!["current"]);
        let config = FsConfig::new(&test_root);

        assert_eq!(*config.current_shell_name().unwrap(), "current".to_string());
    }

    #[test]
    fn can_set_the_current_shell_name() {
        let test_root_dir = set_up("set-current-shell-name", "default", vec!["default"]);
        let test_root = test_root_dir.path();
        let mut config = FsConfig::new(&test_root);
        config.set_current_shell_name("current").unwrap();

        let mut config_file = File::open(&test_root.join("current_shell")).unwrap();
        let mut name_on_disk = String::new();
        config_file.read_to_string(&mut name_on_disk).unwrap();

        let current = "current".to_string();
        assert_eq!(*config.current_shell_name().unwrap(), current);
        assert_eq!(name_on_disk, current);
    }

    #[test]
    fn can_confirm_a_shell_exists() {
        let test_root = set_up(
            "confirm-shell-existence",
            "default",
            vec!["default", "other"],
        );
        let config = FsConfig::new(&test_root);

        assert!(config.shell_exists("other"));
    }

    #[test]
    fn can_confirm_a_shell_does_not_exist() {
        let test_root = set_up(
            "confirm-shell-non-existence",
            "default",
            vec!["default", "other"],
        );
        let config = FsConfig::new(&test_root);

        assert!(!config.shell_exists("another"));
    }

    #[test]
    fn can_walk_a_directory() {
        let test_root = set_up("walk-directory", "default", vec!["default"]);
        let config = FsConfig::new(&test_root);
        let shell_root = config.shell_root_path().join("default");
        File::create(&shell_root.join("file1")).expect("Failed to create test file");

        let files = config
            .shell_files("default")
            .into_iter()
            .map(|f| f.to_string_lossy().to_string())
            .collect::<Vec<_>>();
        assert_eq!(files, vec!["file1"]);
    }

    fn create_paths(
        root_path: impl AsRef<Path>,
        paths: impl IntoIterator<Item = impl AsRef<Path>>,
    ) {
        let root_path = PathBuf::from(root_path.as_ref());
        for path in paths {
            let full_path = root_path.join(path.as_ref());
            let dir_path = full_path.parent().expect("Path had no parent");
            fs::create_dir_all(&dir_path).expect("Failed to create dir path");
            File::create(&full_path).expect("Could not create file");
        }
    }

    #[test]
    fn can_walk_a_directory_skipping_subdirectory_entries() {
        let test_root = set_up(
            "walk-directory-skipping-subdirs",
            "default",
            vec!["default"],
        );
        let config = FsConfig::new(&test_root);
        let shell_root = config.shell_root_path().join("default");
        create_paths(shell_root, vec!["file1", "subdir/file2"]);

        let files = config
            .shell_files("default")
            .into_iter()
            .map(|f| f.to_string_lossy().to_string())
            .collect::<Vec<_>>();
        assert!(files.contains(&"file1".into()));
        assert!(files.contains(&"subdir/file2".into()));
        assert!(!files.contains(&"subdir".into()));
    }
}
