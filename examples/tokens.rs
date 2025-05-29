use elyze::errors::ParseResult;
use elyze::matcher::Match;
use elyze::recognizer::recognize;

fn match_char(c: char, data: &[u8]) -> (bool, usize) {
    match data.first() {
        Some(&d) => (d == c as u8, 1),
        None => (false, 0),
    }
}

enum Token {
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
}

impl Match<u8> for Token {
    fn is_matching(&self, data: &[u8]) -> (bool, usize) {
        match self {
            Token::Plus => match_char('+', data),
            Token::Minus => match_char('-', data),
            Token::Star => match_char('*', data),
            Token::Slash => match_char('/', data),
            Token::LParen => match_char('(', data),
            Token::RParen => match_char(')', data),
        }
    }

    fn size(&self) -> usize {
        match self {
            Token::Plus => 1,
            Token::Minus => 1,
            Token::Star => 1,
            Token::Slash => 1,
            Token::LParen => 1,
            Token::RParen => 1,
        }
    }
}

fn main() -> ParseResult<()> {
    let data = b"((+-)*/)end";
    let mut scanner = elyze::scanner::Scanner::new(data);
    recognize(Token::LParen, &mut scanner)?;
    recognize(Token::LParen, &mut scanner)?;
    recognize(Token::Plus, &mut scanner)?;
    recognize(Token::Minus, &mut scanner)?;
    recognize(Token::RParen, &mut scanner)?;
    recognize(Token::Star, &mut scanner)?;
    recognize(Token::Slash, &mut scanner)?;
    recognize(Token::RParen, &mut scanner)?;

    print!("{:?}", String::from_utf8_lossy(scanner.remaining()));

    Ok(())
}
