use std::error::Error;
use std::{env, fmt};

#[derive(Debug, Clone)]
pub enum AocErrorType {
    InputDataNotFound,
    MissingArgument,
}
#[derive(Debug, Clone)]
pub struct AocError {
    err_type: AocErrorType,
}

impl AocError {
    pub fn new(err: AocErrorType) -> AocError {
        AocError { err_type: err }
    }
}

impl fmt::Display for AocError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AocError: {:?}", self.err_type)
    }
}

impl Error for AocError {}

pub fn get_arg1() -> Option<String> {
    let args: Vec<String> = env::args().collect();
    args.get(1).cloned()
}
