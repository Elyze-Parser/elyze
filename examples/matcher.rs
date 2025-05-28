use elyze::matcher::Match;

struct Hello;
impl Match<u8> for Hello {
    fn is_matching(&self, data: &[u8]) -> (bool, usize) {
        let pattern = b"hello";
        (&data[..pattern.len()] == pattern, pattern.len())
    }

    fn size(&self) -> usize {
        5
    }
}

fn main() {
    let hello = Hello;
    assert_eq!(hello.is_matching(b"hello world"), (true, hello.size()));
    assert_eq!(
        hello.is_matching(b"world is beautiful"),
        (false, hello.size())
    );
}
