use elyze::bytes::primitives::number::{Number, TokenNumber};
use elyze::bytes::token::Token;
use elyze::errors::ParseResult;
use elyze::peek::PeekSize;
use elyze::recognizer::recognize;
use elyze::scanner::Scanner;
use elyze::separated_list::{get_scanner_without_trailing_separator, SeparatedList};
use elyze::visitor::Visitor;

#[derive(Debug)]
struct Separator;

impl<'a> Visitor<'a, u8> for Separator {
    fn accept(scanner: &mut Scanner<u8>) -> ParseResult<Self> {
        recognize(Token::Tilde, scanner)?;
        recognize(Token::Tilde, scanner)?;
        recognize(Token::Tilde, scanner)?;
        Ok(Separator)
    }
}

impl PeekSize for Separator {
    fn peek_size(&self) -> usize {
        3
    }
}

#[derive(Debug)]
struct NumberList {
    data: Vec<usize>,
}

impl<'a> Visitor<'a, u8> for NumberList {
    fn accept(scanner: &mut Scanner<'a, u8>) -> ParseResult<Self> {
        let mut data_scanner =
            get_scanner_without_trailing_separator(TokenNumber, Separator, &scanner)?;

        let data = SeparatedList::<u8, Number<usize>, Separator>::accept(&mut data_scanner)?
            .data
            .into_iter()
            .map(|x| x.0)
            .collect::<Vec<usize>>();

        scanner.bump_by(scanner.data().len());

        Ok(NumberList { data })
    }
}

fn main() -> ParseResult<()> {
    // list of elements separated by a separator
    let data = b"1~~~2~~~3~~~4";
    let mut scanner = Scanner::new(data);

    let result = NumberList::accept(&mut scanner)?;
    println!("{:?}", result); // NumberList { data: [1, 2, 3, 4] }

    // list of elements separated by a separator with trailing separator
    let data = b"1~~~2~~~3~~~4~~~";
    let mut scanner = Scanner::new(data);

    let result = NumberList::accept(&mut scanner)?;
    println!("{:?}", result); // NumberList { data: [1, 2, 3, 4] }

    // list of 1 element with trailing separator
    let data = b"1~~~";
    let mut scanner = Scanner::new(data);

    let result = NumberList::accept(&mut scanner)?;
    println!("{:?}", result); // NumberList { data: [1] }

    // list of 1 element without trailing separator
    let data = b"1";
    let mut scanner = Scanner::new(data);

    let result = NumberList::accept(&mut scanner)?;
    println!("{:?}", result); // NumberList { data: [1] }

    // list of 0 elements
    let data = b"";
    let mut scanner = Scanner::new(data);

    let result = NumberList::accept(&mut scanner)?;
    println!("{:?}", result); // NumberList { data: [] }

    // bad data
    let data = b"bad~~~";
    let mut scanner = Scanner::new(data);

    let result = NumberList::accept(&mut scanner);
    println!("{:?}", result); // Err(UnexpectedToken)

    Ok(())
}
