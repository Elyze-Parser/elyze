use crate::bytes::matchers::match_char;
use crate::matcher::Match;

/// The token type
pub enum Token {
    /// The "(" character
    OpenParen,
    /// The `)` character
    CloseParen,
    /// The `,` character
    Comma,
    /// The `;` character
    Semicolon,
    /// The `:` character
    Colon,
    /// The whitespace character
    Whitespace,
    /// The `>` character
    GreaterThan,
    /// The `<` character
    LessThan,
    /// The `!` character
    Exclamation,
    /// The `'` character
    Quote,
    /// The `"` character
    DoubleQuote,
}

impl Match<u8> for Token {
    fn matcher(&self, data: &[u8]) -> (bool, usize) {
        match self {
            Token::OpenParen => match_char('(', data),
            Token::CloseParen => match_char(')', data),
            Token::Comma => match_char(',', data),
            Token::Semicolon => match_char(';', data),
            Token::Colon => match_char(':', data),
            Token::Whitespace => match_char(' ', data),
            Token::GreaterThan => match_char('>', data),
            Token::LessThan => match_char('<', data),
            Token::Exclamation => match_char('!', data),
            Token::Quote => match_char('\'', data),
            Token::DoubleQuote => match_char('"', data),
        }
    }

    fn size(&self) -> usize {
        match self {
            Token::OpenParen => 1,
            Token::CloseParen => 1,
            Token::Comma => 1,
            Token::Semicolon => 1,
            Token::Colon => 1,
            Token::Whitespace => 1,
            Token::GreaterThan => 1,
            Token::LessThan => 1,
            Token::Exclamation => 1,
            Token::Quote => 1,
            Token::DoubleQuote => 1,
        }
    }
}
