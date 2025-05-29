use elyze::errors::ParseResult;
use elyze::matcher::Match;
use elyze::peek::{peek, PeekResult, Peekable};
use elyze::recognizer::Recognizable;
use elyze::scanner::Scanner;

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

struct ParenthesesGroup;

impl<'a> Peekable<'a, u8> for ParenthesesGroup {
    fn peek(&self, scanner: &Scanner<'a, u8>) -> ParseResult<PeekResult> {
        // create an internal scanner allowing to peek data without alterating the original scanner
        let mut inner_scanner = Scanner::new(&scanner.remaining());

        // loop on each byte until we find a close parenthesis
        loop {
            if inner_scanner.is_empty() {
                // we have reached the end without finding a close parenthesis
                break;
            }
            if CloseParentheses.recognize(&mut inner_scanner)?.is_some() {
                // we have found a close parenthesis
                return Ok(PeekResult::Found {
                    // we return the position of the close parenthesis
                    end_slice: inner_scanner.current_position(),
                    // our peeking doesn't include a start element
                    start_element_size: 0,
                    // the size of the end element is a close parenthesis of 1 byte
                    end_element_size: 1,
                });
            }

            // consume the current byte
            inner_scanner.bump_by(1);
        }

        // At this point, we have reached the end of available data without finding a close parenthesis
        Ok(PeekResult::NotFound)
    }
}

fn main() -> ParseResult<()> {
    let data = b"7 * ( 1 + 2 )";
    let mut scanner = Scanner::new(data);
    scanner.bump_by(5); // consumes : 7 * (
    let result = peek(ParenthesesGroup, &scanner)?;
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
