use elyze::errors::{ParseError, ParseResult};
use elyze::matcher::Match;
use elyze::peek::{peek, PeekResult, Peekable, Until};
use elyze::peeker::Peeker;
use elyze::recognizer::Recognizer;
use elyze::scanner::Scanner;
use elyze::visitor::Visitor;

fn match_char(data: &[u8], c: u8) -> (bool, usize) {
    (data[0] == c, 1)
}

enum OperatorTokens {
    Plus,
    Times,
}

impl Match<u8> for OperatorTokens {
    fn is_matching(&self, data: &[u8]) -> (bool, usize) {
        match self {
            OperatorTokens::Plus => match_char(data, b'+'),
            OperatorTokens::Times => match_char(data, b'*'),
        }
    }

    fn size(&self) -> usize {
        match self {
            OperatorTokens::Plus => 1,
            OperatorTokens::Times => 1,
        }
    }
}

impl<'a> Visitor<'a, u8> for OperatorTokens {
    fn accept(scanner: &mut Scanner<'a, u8>) -> ParseResult<Self> {
        Ok(Recognizer::new(scanner)
            .try_or(OperatorTokens::Times)?
            .try_or(OperatorTokens::Plus)?
            .finish()
            .ok_or(ParseError::UnexpectedToken)?)
    }
}

struct FirstOperator;

impl<'a> Peekable<'a, u8> for FirstOperator {
    fn peek(&self, scanner: &Scanner<'a, u8>) -> ParseResult<PeekResult> {
        Peeker::new(scanner)
            .add_peekable(Until::new(OperatorTokens::Plus))
            .add_peekable(Until::new(OperatorTokens::Times))
            .peek()
            .map(Into::into)
    }
}

fn main() -> ParseResult<()> {
    let data = b"7 * ( 1 + 2 )";
    let scanner = Scanner::new(data);
    let slice = peek(FirstOperator, &scanner)?;
    if let Some(slice) = slice {
        println!("{:?}", String::from_utf8_lossy(slice.peeked_slice())); // "7 "
    }

    let data = b"7 * ( 1 + 2 )";
    let scanner = Scanner::new(data);
    let slice = peek(FirstOperator, &scanner)?;
    if let Some(slice) = slice {
        println!("{:?}", String::from_utf8_lossy(slice.peeked_slice())); // "7 "
    }

    let data = b"1 + 2 * 7";
    let scanner = Scanner::new(data);
    let slice = peek(FirstOperator, &scanner)?;
    if let Some(slice) = slice {
        println!("{:?}", String::from_utf8_lossy(slice.peeked_slice())); // "1 "
    }

    Ok(())
}
