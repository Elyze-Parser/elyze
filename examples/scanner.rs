use elyze::scanner::Scanner;

fn process(mut scanner: Scanner<'_, u8>) -> &[u8] {
    // do something with the data
    // then bump the scanner
    scanner.bump_by(3);
    // return the remaining
    scanner.remaining()
}

fn main() {
    let data = b"hello world";
    let remaining = process(Scanner::new(data));
    assert_eq!(remaining, b"lo world");
}
