use elyze::bytes::primitives::string::DataString;
use elyze::bytes::primitives::whitespace::Whitespaces;
use elyze::errors::ParseResult;
use elyze::scanner::Scanner;
use elyze::visitor::Visitor;

#[derive(Debug)]
#[allow(dead_code)]
struct Data<'a>(&'a str);

impl<'a> Visitor<'a, u8> for Data<'a> {
    fn accept(scanner: &mut Scanner<'a, u8>) -> ParseResult<Self> {
        // consume whitespaces
        Whitespaces::accept(scanner)?;
        // before parse string
        let raw_data = DataString::accept(scanner)?.0;
        Ok(Data(raw_data))
    }
}

fn main() {
    let data = b"     data    ";
    let mut scanner = Scanner::new(data);
    let result = Data::accept(&mut scanner);
    println!("{:?}", result); // Ok(Data("data"))
}
