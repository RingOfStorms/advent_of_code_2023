pub type SResult<T, R> = std::result::Result<T, R>;
pub type SError = dyn std::error::Error;
pub type BoxE = Box<SError>;
pub type Result<T> = SResult<T, BoxE>;
