use elyze::errors::ParseResult;
use elyze::matcher::Match;
use elyze::peek::{peek, DefaultPeekableImplementation, Last, PeekableImplementation};
use elyze::scanner::Scanner;

#[derive(Default)]
struct CloseParentheses;

impl Match<u8> for CloseParentheses {
    fn is_matching(&self, data: &[u8]) -> (bool, usize) {
        if data[0] == b')' {
            (true, 1)
        } else {
            (false, 0)
        }
    }

    fn size(&self) -> usize {
        1
    }
}

impl PeekableImplementation for CloseParentheses {
    type Type = DefaultPeekableImplementation;
}

fn main() -> ParseResult<()> {
    let data = b"8 / ( 7 * ( 1 + 2 ) )";
    let mut scanner = Scanner::new(data);
    // consumes : "8 / ( " to reach the start of the enclosed data
    scanner.bump_by(b"8 / (".len());
    let result = peek(Last::new(CloseParentheses), &scanner)?;
    if let Some(peeking) = result {
        println!(
            "{:?}",
            // the peek_slice method returns the all enclosed data
            String::from_utf8_lossy(peeking.peeked_slice()) //  7 * ( 1 + 2 )
        );
    }
    Ok(())
}
