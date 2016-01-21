use env;

pub fn error(details: &str) -> String {
    format!("{}: error: {}", env::get_program_name(), details)
}
