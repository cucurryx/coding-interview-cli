use std::error;
use std::fmt;

/// work with multiple error types
pub type GenError = Box<error::Error>;
pub type GenResult<T> = Result<T, GenError>;

#[derive(Debug, Clone)]
pub struct CodeRootError;

impl CodeRootError {}

impl fmt::Display for CodeRootError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid code root")
    }
}

impl error::Error for CodeRootError {
    fn description(&self) -> &str {
        "invalid code root"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}
