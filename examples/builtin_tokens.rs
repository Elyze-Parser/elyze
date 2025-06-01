use elyze::bytes::token::Token;
use elyze::errors::{ParseError, ParseResult};
use elyze::peek::{peek, Last};
use elyze::recognizer::{recognize, Recognizer};
use elyze::scanner::Scanner;
use elyze::visitor::Visitor;

fn main() -> ParseResult<()> {
    let data = b"+-*";

    // use recognize
    let mut scanner = Scanner::new(data);
    let recognized = recognize(Token::Plus, &mut scanner)?;
    assert_eq!(recognized, Token::Plus);

    // use the recognizer
    let mut scanner = Scanner::new(data);
    let recognized = Recognizer::new(&mut scanner)
        .try_or(Token::Dash)?
        .try_or(Token::Plus)?
        .try_or(Token::Star)?
        .finish()
        .ok_or(ParseError::UnexpectedToken)?;
    assert_eq!(recognized, Token::Plus);

    // use the visitor
    let mut scanner = Scanner::new(data);
    let accepted = Token::accept(&mut scanner)?;
    assert_eq!(accepted, Token::Plus);

    // use peek
    let mut scanner = Scanner::new(data);
    let peeked = peek(Token::Dash, &mut scanner)?;
    if let Some(peeked) = peeked {
        assert_eq!(peeked.peeked_slice(), b"+");
    }

    // last token
    let data = b" 8 + ( 7 * ( 1 + 2 ) )";
    let mut scanner = Scanner::new(data);
    let peeked = peek(Last::new(Token::CloseParen), &mut scanner)?;
    if let Some(peeked) = peeked {
        assert_eq!(peeked.peeked_slice(), b" 8 + ( 7 * ( 1 + 2 ) ");
    }

    Ok(())
}
