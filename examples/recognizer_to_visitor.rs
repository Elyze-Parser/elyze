use elyze::errors::{ParseError, ParseResult};
use elyze::matcher::Match;
use elyze::recognizer::Recognizer;
use elyze::scanner::Scanner;
use elyze::visitor::Visitor;

#[derive(Debug)]
enum Operator {
    Add,
    Sub,
}

fn match_char(c: char, data: &[u8]) -> (bool, usize) {
    match data.first() {
        Some(&d) => (d == c as u8, 1),
        None => (false, 0),
    }
}

impl Match<u8> for Operator {
    fn is_matching(&self, data: &[u8]) -> (bool, usize) {
        match self {
            Operator::Add => match_char('+', data),
            Operator::Sub => match_char('-', data),
        }
    }

    fn size(&self) -> usize {
        match self {
            Operator::Add => 1,
            Operator::Sub => 1,
        }
    }
}

#[derive(Debug)]
// Define a structure to implement the `Visitor` trait
struct OperatorData(Operator);

impl<'a> Visitor<'a, u8> for OperatorData {
    fn accept(scanner: &mut Scanner<'a, u8>) -> ParseResult<Self> {
        // Build and apply the recognizer
        let operator = Recognizer::new(scanner)
            .try_or(Operator::Add)?
            .try_or(Operator::Sub)?
            .finish()
            // If the recognizer fails, return an error
            .ok_or(ParseError::UnexpectedToken)?;

        Ok(OperatorData(operator))
    }
}

fn main() -> ParseResult<()> {
    let data = b"+";
    let mut scanner = Scanner::new(data);
    // Initialize the recognizer
    let result = OperatorData::accept(&mut scanner)?.0;
    dbg!(result); // Operator::Add

    let data = b"-";
    let mut scanner = Scanner::new(data);
    // Initialize the recognizer
    let result = OperatorData::accept(&mut scanner)?.0;
    dbg!(result); // Operator::Sub

    let data = b"x";
    let mut scanner = Scanner::new(data);
    // Initialize the recognizer
    let result = OperatorData::accept(&mut scanner);
    assert!(matches!(result, Err(ParseError::UnexpectedToken))); // Err(UnexpectedToken)

    Ok(())
}
