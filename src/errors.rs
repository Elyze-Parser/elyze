//! Error types
/// The result of a parse operation
pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    /// The parser reached the end of the input
    #[error("Unexpected end of input")]
    UnexpectedEndOfInput,
    #[error("Unexpected token have been encountered")]
    /// The parser encountered an unexpected token
    UnexpectedToken,
    /// Unable to decode a string as UTF-8
    #[error("UTF-8 error: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),
    /// Unable to parse an integer from a string
    #[error("ParseIntError: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
}
