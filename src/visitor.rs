use crate::errors::ParseResult;
use crate::scanner::Scanner;

pub trait Visitor<'a, T>: Sized {
    fn accept(scanner: &mut Scanner<'a, T>) -> ParseResult<Self>;
}
