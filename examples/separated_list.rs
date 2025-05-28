use elyze::bytes::primitives::number::Number;
use elyze::bytes::token::Token;
use elyze::errors::ParseResult;
use elyze::recognizer::recognize;
use elyze::scanner::Scanner;
use elyze::separated_list::SeparatedList;
use elyze::visitor::Visitor;

#[derive(Debug)]
struct Separator;

impl<'a> Visitor<'a, u8> for Separator {
    fn accept(scanner: &mut elyze::scanner::Scanner<u8>) -> ParseResult<Self> {
        recognize(Token::Tilde, scanner)?;
        recognize(Token::Tilde, scanner)?;
        recognize(Token::Tilde, scanner)?;
        Ok(Separator)
    }
}

fn main() {
    let data = b"1~~~2~~~3~~~4";
    let mut scanner = Scanner::new(data);
    let result =
        SeparatedList::<u8, Number<usize>, Separator>::accept(&mut scanner).map(|x| x.data);
    println!("{:?}", result); // Ok([Number(1), Number(2), Number(3), Number(4)])
}
