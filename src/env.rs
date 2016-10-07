use std::env;
use std::path::PathBuf;

pub use std::env::home_dir;

pub fn get_program_name() -> String {
    env::args()
        .nth(0)
        .map(PathBuf::from)
        .and_then(|path| path.file_name().map(|name| name.to_owned()))
        .and_then(|file_name| file_name.to_str().map(|name| name.to_owned()))
        .unwrap_or("hermit".to_owned())
}

pub fn get_hermit_dir() -> Option<PathBuf> {
    env::var("HERMIT_ROOT")
        .map(PathBuf::from)
        .ok()
        .or_else(default_hermit_dir)
}

pub fn default_hermit_dir() -> Option<PathBuf> {
    env::home_dir().map(|home| home.join(".config/hermit"))
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::env;
    use std::thread;
    use std::time::Duration;
    use std::path::PathBuf;

    #[test]
    fn hermit_dir_defaults_to_dot_config() {
        env::remove_var("HERMIT_ROOT");
        assert_eq!(default_hermit_dir(), get_hermit_dir());
    }

    #[test]
    fn hermit_dir_can_be_set_by_environment_variable() {
        // Sleep briefly so that this doesn't influence the default
        // hermit dir test.
        thread::sleep(Duration::from_millis(500));

        let hermit_dir = PathBuf::from("a/hermit/path");
        env::set_var("HERMIT_ROOT", &hermit_dir);

        assert_eq!(Some(hermit_dir), get_hermit_dir());
    }
}
