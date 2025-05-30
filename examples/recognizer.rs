use elyze::errors::ParseResult;
use elyze::matcher::Match;
use elyze::recognizer::Recognizer;
use elyze::scanner::Scanner;

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

fn main() -> ParseResult<()> {
    let data = b"+";
    let mut scanner = Scanner::new(data);
    // Initialize the recognizer
    let recognizer = Recognizer::new(&mut scanner);
    // Try to apply the recognizer on the operator add, if it fails, return an error
    let recognizer_add = recognizer.try_or(Operator::Add)?;
    // Try to apply the recognizer on the operator sub, if it fails, return an error
    let recognizer_add_and_sub = recognizer_add.try_or(Operator::Sub)?;
    // Finish the recognizer
    let result = recognizer_add_and_sub.finish();
    dbg!(result);

    Ok(())
}
