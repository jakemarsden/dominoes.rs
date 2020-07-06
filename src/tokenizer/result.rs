use super::error::ParseError;

pub type Result<T> = std::result::Result<T, ParseError>;
