use elyze::matcher::Match;
use elyze::recognizer::Recognizable;
use elyze::scanner::Scanner;

// define a structure to implement the `Match` trait
struct UntilFirstSpace;

// implement the `Match` trait
impl Match<u8> for UntilFirstSpace {
    fn is_matching(&self, data: &[u8]) -> (bool, usize) {
        let mut pos = 0;
        while pos < data.len() && data[pos] != b' ' {
            pos += 1;
        }
        (pos > 0, pos)
    }

    // The size of the object is unknown
    fn size(&self) -> usize {
        0
    }
}

fn main() {
    let mut scanner = Scanner::new(b"hello world");
    let result = UntilFirstSpace
        .recognize_slice(&mut scanner)
        .expect("failed to parse");
    println!("{:?}", result.map(|s| String::from_utf8_lossy(s))); // Some("hello")

    let mut scanner = Scanner::new(b"loooooooooong string");
    let result = UntilFirstSpace
        .recognize_slice(&mut scanner)
        .expect("failed to parse");
    println!("{:?}", result.map(|s| String::from_utf8_lossy(s))); // Some("loooooooooong")
}
