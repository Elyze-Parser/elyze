//! Specialized primitive parsers for bytes.

use crate::matcher::Match;
use crate::peek::Until;

pub mod binary_operator;
pub mod number;
pub mod string;
pub mod whitespace;

impl<'a, M: Match<u8>> Match<u8> for Until<'a, u8, M> {
    fn is_matching(&self, data: &[u8]) -> (bool, usize) {
        self.element.is_matching(data)
    }

    fn size(&self) -> usize {
        self.element.size()
    }
}
