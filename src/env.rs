use std::borrow::ToOwned;
use std::env;
use std::path::PathBuf;

pub use dirs::home_dir;

pub fn get_program_name() -> String {
    env::args()
        .nth(0)
        .map(PathBuf::from)
        .and_then(|path| path.file_name().map(ToOwned::to_owned))
        .and_then(|file_name| file_name.to_str().map(ToOwned::to_owned))
        .unwrap_or_else(|| "hermit".to_owned())
}

pub fn get_hermit_dir() -> Option<PathBuf> {
    env::var("HERMIT_ROOT")
        .map(PathBuf::from)
        .ok()
        .or_else(default_hermit_dir)
}

pub fn default_hermit_dir() -> Option<PathBuf> {
    dirs::config_dir().map(|home| home.join("hermit"))
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::env;
    use std::path::PathBuf;
    use std::sync::Mutex;

    lazy_static! {
        // This mutex is solely for preventing these two tests from
        // stomping on each other. While it doesn't happen often, it's
        // still an issue we want to avoid for CI builds (spurious
        // build failures are the worst).
        static ref ROOT_ENV_LOCK: Mutex<()> = Mutex::new(());
    }

    #[test]
    fn hermit_dir_defaults_to_dot_config() {
        let hermit_dir: Option<PathBuf>;
        {
            let _lock = ROOT_ENV_LOCK.lock().unwrap();
            env::remove_var("HERMIT_ROOT");
            hermit_dir = get_hermit_dir();
        }
        assert_eq!(default_hermit_dir(), hermit_dir);
    }

    #[test]
    fn hermit_dir_can_be_set_by_environment_variable() {
        let hermit_dir: Option<PathBuf>;
        let test_hermit_dir = PathBuf::from("a/hermit/path");
        {
            let _lock = ROOT_ENV_LOCK.lock().unwrap();
            env::set_var("HERMIT_ROOT", &test_hermit_dir);
            hermit_dir = get_hermit_dir();
        }

        assert_eq!(Some(test_hermit_dir), hermit_dir);
    }
}
