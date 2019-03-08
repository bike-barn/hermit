use failure::{self, Error};

use crate::env;

pub fn error_str<T: 'static + Into<String>>(details: T) -> String {
    error(failure::err_msg(details.into()))
}

pub fn error(failure: impl Into<Error>) -> String {
    format!("{}: error: {}", env::get_program_name(), failure.into())
}
