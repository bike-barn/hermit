pub trait HermitConfig {
    fn current_shell_name(&self) -> String;

    fn set_current_shell_name(&mut self, name: &str);

    fn does_shell_exist(&self, name: &str) -> bool;
}

#[cfg(test)]
pub mod mock {
    use super::HermitConfig;

    #[derive(Clone)]
    pub struct MockConfig {
        pub default_shell: String,
        pub allowed_shell_names: Vec<String>,
    }

    impl HermitConfig for MockConfig {
        fn current_shell_name(&self) -> String {
            self.default_shell.clone()
        }

        fn set_current_shell_name(&mut self, name: &str) {
            self.default_shell = name.to_string();
        }

        fn does_shell_exist(&self, name: &str) -> bool {
            self.allowed_shell_names.contains(&name.to_string())
        }
    }
}
