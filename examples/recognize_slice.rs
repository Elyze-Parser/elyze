use elyze::errors::ParseResult;
use elyze::matcher::Match;
use elyze::recognizer::recognize_slice;
use elyze::scanner::Scanner;

// define a structure to implement the `Match` trait
struct Hello;

// implement the `Match` trait
impl Match<u8> for Hello {
    fn is_matching(&self, data: &[u8]) -> (bool, usize) {
        // define the pattern to match
        let pattern = b"hello";
        // check if the subslice of data matches the pattern
        (&data[..pattern.len()] == pattern, pattern.len())
    }

    fn size(&self) -> usize {
        5
    }
}

fn main() -> ParseResult<()> {
    let mut scanner = Scanner::new(b"hello world");
    let hello_string = recognize_slice(Hello, &mut scanner)?;

    println!("found: {}", String::from_utf8_lossy(hello_string)); // found: "hello"
    print!(
        "remaining: {:?}",
        String::from_utf8_lossy(scanner.remaining())
    ); // remaining: " world"

    Ok(())
}
