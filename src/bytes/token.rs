//! Classic tokens

use crate::bytes::matchers::{match_char, match_pattern};
use crate::errors::{ParseError, ParseResult};
use crate::matcher::Match;
use crate::peek;
use crate::recognizer::Recognizer;
use crate::scanner::Scanner;
use crate::visitor::Visitor;

#[derive(Copy, Clone)]
/// The token type
#[derive(PartialEq, Debug)]
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
    /// The `=` character
    Equal,
    /// The `+` character
    Plus,
    /// The `-` character
    Dash,
    /// The `/` character
    Slash,
    /// The `*` character
    Star,
    /// The `%` character
    Percent,
    /// The `&` character
    Ampersand,
    /// The `|` character
    Pipe,
    /// The `^` character
    Caret,
    /// The `~` character
    Tilde,
    /// The `.` character
    Dot,
    /// The `?` character
    Question,
    /// The `@` character
    At,
    /// The `#` character
    Hash,
    /// The `$` character
    Dollar,
    /// The `\\` character
    Backslash,
    /// The `_` character
    Underscore,
    /// The `#` character
    Sharp,
    /// The `\n` character
    Ln,
    /// The `\r` character
    Cr,
    /// The `\t` character
    Tab,
    /// The `\r\n` character
    CrLn,
}

impl Match<u8> for Token {
    fn is_matching(&self, data: &[u8]) -> (bool, usize) {
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
            Token::Equal => match_char('=', data),
            Token::Plus => match_char('+', data),
            Token::Dash => match_char('-', data),
            Token::Slash => match_char('/', data),
            Token::Star => match_char('*', data),
            Token::Percent => match_char('%', data),
            Token::Ampersand => match_char('&', data),
            Token::Pipe => match_char('|', data),
            Token::Caret => match_char('^', data),
            Token::Tilde => match_char('~', data),
            Token::Dot => match_char('.', data),
            Token::Question => match_char('?', data),
            Token::At => match_char('@', data),
            Token::Hash => match_char('#', data),
            Token::Dollar => match_char('$', data),
            Token::Backslash => match_char('\\', data),
            Token::Underscore => match_char('_', data),
            Token::Sharp => match_char('#', data),
            Token::Ln => match_char('\n', data),
            Token::Cr => match_char('\r', data),
            Token::Tab => match_char('\t', data),
            Token::CrLn => match_pattern(b"\r\n", data),
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
            Token::Equal => 1,
            Token::Plus => 1,
            Token::Dash => 1,
            Token::Slash => 1,
            Token::Star => 1,
            Token::Percent => 1,
            Token::Ampersand => 1,
            Token::Pipe => 1,
            Token::Caret => 1,
            Token::Tilde => 1,
            Token::Dot => 1,
            Token::Question => 1,
            Token::At => 1,
            Token::Hash => 1,
            Token::Dollar => 1,
            Token::Backslash => 1,
            Token::Underscore => 1,
            Token::Sharp => 1,
            Token::Ln => 1,
            Token::Cr => 1,
            Token::Tab => 1,
            Token::CrLn => 2,
        }
    }
}

/// Implement Visitor for Token make it possible to use Token::accept
///
/// Make it also usable with [peek::Until]
impl<'a> Visitor<'a, u8> for Token {
    fn accept(scanner: &mut Scanner<'a, u8>) -> ParseResult<Self> {
        Recognizer::new(scanner)
            .try_or(Token::OpenParen)?
            .try_or(Token::CloseParen)?
            .try_or(Token::Comma)?
            .try_or(Token::Semicolon)?
            .try_or(Token::Colon)?
            .try_or(Token::Whitespace)?
            .try_or(Token::GreaterThan)?
            .try_or(Token::LessThan)?
            .try_or(Token::Exclamation)?
            .try_or(Token::Quote)?
            .try_or(Token::DoubleQuote)?
            .try_or(Token::Equal)?
            .try_or(Token::Plus)?
            .try_or(Token::Dash)?
            .try_or(Token::Slash)?
            .try_or(Token::Star)?
            .try_or(Token::Percent)?
            .try_or(Token::Ampersand)?
            .try_or(Token::Pipe)?
            .try_or(Token::Caret)?
            .try_or(Token::Tilde)?
            .try_or(Token::Dot)?
            .try_or(Token::Question)?
            .try_or(Token::At)?
            .try_or(Token::Hash)?
            .try_or(Token::Dollar)?
            .try_or(Token::Backslash)?
            .try_or(Token::Underscore)?
            .try_or(Token::Sharp)?
            .try_or(Token::Ln)?
            .try_or(Token::Cr)?
            .try_or(Token::Tab)?
            .try_or(Token::CrLn)?
            .finish()
            .ok_or(ParseError::UnexpectedToken)
    }
}
