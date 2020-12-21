use std::fmt::Display;

use crate::env;

pub fn error_str<T: 'static + Into<String>>(details: T) -> String {
    error(anyhow::Error::msg(details.into()))
}

pub fn error(failure: impl Display) -> String {
    format!("{}: error: {}", env::get_program_name(), failure)
}
