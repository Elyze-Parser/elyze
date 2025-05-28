use elyze::bytes::matchers::match_pattern;
use elyze::errors::{ParseError, ParseResult};
use elyze::matcher::Match;
use elyze::recognizer::Recognizer;
use elyze::scanner::Scanner;

#[derive(Debug)]
enum OperatorTokens {
    /// The `==` operator.
    Equal,
    /// The `!=` operator.
    NotEqual,
}

impl Match<u8> for OperatorTokens {
    fn is_matching(&self, data: &[u8]) -> (bool, usize) {
        match self {
            OperatorTokens::Equal => match_pattern(b"==", data),
            OperatorTokens::NotEqual => match_pattern(b"!=", data),
        }
    }

    fn size(&self) -> usize {
        match self {
            OperatorTokens::Equal => 2,
            OperatorTokens::NotEqual => 2,
        }
    }
}

fn main() -> ParseResult<()> {
    let data = b"== 2";
    let mut scanner = Scanner::new(data);
    let recognized = Recognizer::new(&mut scanner)
        .try_or(OperatorTokens::NotEqual)?
        .try_or(OperatorTokens::Equal)?
        .finish()
        .ok_or(ParseError::UnexpectedToken)?;

    println!("{:?}", recognized); // ==

    let data = b"!= 2";
    let mut scanner = Scanner::new(data);
    let recognized = Recognizer::new(&mut scanner)
        .try_or(OperatorTokens::NotEqual)?
        .try_or(OperatorTokens::Equal)?
        .finish()
        .ok_or(ParseError::UnexpectedToken)?;

    println!("{:?}", recognized); // !=

    let data = b"> 2";
    let mut scanner = Scanner::new(data);
    let recognized = Recognizer::new(&mut scanner)
        .try_or(OperatorTokens::NotEqual)?
        .try_or(OperatorTokens::Equal)?
        .finish()
        .ok_or(ParseError::UnexpectedToken);

    println!("{:?}", recognized); // error (UnexpectedToken)

    Ok(())
}
