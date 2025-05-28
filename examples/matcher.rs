use elyze::matcher::Match;

struct Hello;
impl Match<u8> for Hello {
    fn matcher(&self, data: &[u8]) -> (bool, usize) {
        let pattern = b"hello";
        (&data[..pattern.len()] == pattern, pattern.len())
    }
}

fn main() {
    let hello = Hello;
    assert_eq!(hello.matcher(b"hello world"), (true, 5));
    assert_eq!(hello.matcher(b"world is beautiful"), (false, 5));
}
