/// work with multiple error types
pub type GenError = Box<std::error::Error>;
pub type GenResult<T> = Result<T, GenError>;
