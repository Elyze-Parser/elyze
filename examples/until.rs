use elyze::errors::ParseResult;
use elyze::matcher::Match;
use elyze::peek::{peek, Until};
use elyze::scanner::Scanner;

#[derive(Clone)]
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

fn main() -> ParseResult<()> {
    let data = b"7 * ( 1 + 2 )";
    let mut scanner = Scanner::new(data);
    scanner.bump_by(5); // consumes : 7 * (
    let result = peek(Until::new(CloseParentheses), &scanner)?;
    if let Some(peeking) = result {
        println!(
            "{:?}",
            // the peek_slice method returns the slice of recognized without the end element
            String::from_utf8_lossy(peeking.peeked_slice()) // 1 + 2
        );
    } else {
        println!("not found");
    }
    println!(
        "scanner: {:?}",
        // the scanner itself remains unchanged
        String::from_utf8_lossy(scanner.remaining()) // scanner: " 1 + 2 )"
    );
    Ok(())
}
