use elyze::bytes::matchers::match_number;
use elyze::matcher::Match;
use elyze::recognizer::Recognizable;

struct TokenNumber;

/// Implement the `Match` trait for the token number.
impl Match<u8> for TokenNumber {
    fn is_matching(&self, data: &[u8]) -> (bool, usize) {
        match_number(data)
    }

    fn size(&self) -> usize {
        0
    }
}

fn main() {
    let data = b"123abc";
    let mut scanner = elyze::scanner::Scanner::new(data);
    let result = TokenNumber.recognize_slice(&mut scanner);
    println!("{:?}", result); // Ok(Some([49, 50, 51]))
    // If the result is successful
    if let Ok(Some(data)) = result {
        // Convert the data to a string
        let str_data = std::str::from_utf8(data).unwrap();
        // Convert the string to a number
        let result = str_data.parse::<usize>().unwrap();
        println!("{}", result); // 123
    }
}
