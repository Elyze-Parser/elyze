/// The result of a parse operation
pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Unexpected end of input")]
    UnexpectedEndOfInput,
    #[error("Unexpected token have been encountered")]
    UnexpectedToken,
}
