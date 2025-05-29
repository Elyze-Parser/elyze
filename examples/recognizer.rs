use elyze::matcher::Match;
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

fn main() {
    let mut scanner = Scanner::new(b"hello world");
    let (found, size) = Hello.is_matching(scanner.remaining());
    if !found {
        println!("not found");
        return;
    }
    let data = &scanner.remaining()[..size];
    scanner.bump_by(size);
    println!("found: {:?}", String::from_utf8_lossy(data));
}
