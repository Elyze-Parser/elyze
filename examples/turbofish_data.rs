use elyze::bytes::primitives::number::Number;
use elyze::bytes::token::Token;
use elyze::errors::ParseResult;
use elyze::recognizer::recognize;
use elyze::visitor::Visitor;

#[derive(Debug)]
#[allow(dead_code)]
struct Turbofish(usize);

// Implement the `Visitor` trait for the turbofish operator.
impl<'a> Visitor<'a, u8> for Turbofish {
    fn accept(scanner: &mut elyze::scanner::Scanner<u8>) -> ParseResult<Self> {
        // recognize the turbofish operator start "::<".
        recognize(Token::Colon, scanner)?;
        recognize(Token::Colon, scanner)?;
        recognize(Token::LessThan, scanner)?;
        // recognize the number
        let number = Number::accept(scanner)?.0;
        // recognize the turbofish operator end ">"
        recognize(Token::GreaterThan, scanner)?;
        Ok(Turbofish(number))
    }
}

fn main() {
    let data = b"::<45>garbage";
    let mut scanner = elyze::scanner::Scanner::new(data);
    let result = Turbofish::accept(&mut scanner);
    println!("{:?}", result); // Ok(Turbofish(45))
}
