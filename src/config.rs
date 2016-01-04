use std::path::PathBuf;

pub trait HermitConfig {
    fn root_path(&self) -> &PathBuf;

    fn current_shell_name(&self) -> String;

    fn set_current_shell_name(&mut self, name: &str);

    fn does_shell_exist(&self, name: &str) -> bool;
}

#[cfg(test)]
pub mod mock {
    use std::path::PathBuf;

    use super::HermitConfig;

    #[derive(Clone)]
    pub struct MockConfig {
        pub root_path: PathBuf,
        pub current_shell: String,
        pub allowed_shell_names: Vec<String>,
    }

    impl HermitConfig for MockConfig {
        fn root_path(&self) -> &PathBuf {
            &self.root_path
        }

        fn current_shell_name(&self) -> String {
            self.current_shell.clone()
        }

        fn set_current_shell_name(&mut self, name: &str) {
            self.current_shell = name.to_string();
        }

        fn does_shell_exist(&self, name: &str) -> bool {
            self.allowed_shell_names.contains(&name.to_string())
        }
    }
}
