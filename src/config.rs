pub trait HermitConfig {
    fn current_shell_name(&self) -> String;
}

#[cfg(test)]
pub mod mock {
    use super::HermitConfig;

    #[derive(Clone)]
    pub struct MockConfig {
        pub default_shell: String,
    }

    impl HermitConfig for MockConfig {
        fn current_shell_name(&self) -> String {
            self.default_shell.clone()
        }
    }
}
