//! String primitives

use crate::bytes::matchers::match_string;
use crate::errors::ParseResult;
use crate::matcher::Match;
use crate::recognizer::recognize_slice;
use crate::scanner::Scanner;
use crate::visitor::Visitor;
use std::borrow::Cow;

struct TokenString;

impl Match<u8> for TokenString {
    fn is_matching(&self, data: &[u8]) -> (bool, usize) {
        match_string(data)
    }

    fn size(&self) -> usize {
        0
    }
}

pub struct DataString<T>(pub T);

/// Implement the `Visitor` trait for the token string.
macro_rules! impl_string {
    ($type:ty, $a:lifetime) => {
        impl<$a> Visitor<$a, u8> for DataString<$type> {
            fn accept(scanner: &mut Scanner<$a, u8>) -> ParseResult<Self> {
                let raw_data = recognize_slice(TokenString, scanner)?;
                let str_data = std::str::from_utf8(raw_data)?;
                Ok(DataString(str_data.into()))
            }
        }
    };
}

impl_string!(&'a str, 'a);
impl_string!(String, 'a);
impl_string!(Cow<'a, str>, 'a);
