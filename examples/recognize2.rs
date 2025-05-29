use elyze::matcher::Match;
use elyze::recognizer::Recognizable;
use elyze::scanner::Scanner;

// define a structure to implement the `Match` trait
#[derive(Debug)]
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

fn main() {
    let mut scanner = Scanner::new(b"hello world");
    let data = Hello.recognize(&mut scanner).expect("failed to parse");

    if let Some(hello) = data {
        println!("found: {hello:?}"); // found: "Hello"
        print!(
            "remaining: {:?}",
            String::from_utf8_lossy(scanner.remaining())
        ); // remaining: " world"
    } else {
        println!("not found");
    }
}
