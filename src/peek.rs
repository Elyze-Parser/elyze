//! Peekable types
//!
//! A `Peekable` is a type that can be used to peek at the current position of a
//! `Scanner` without advancing the scanner.

use crate::errors::ParseResult;
use crate::matcher::Match;
use crate::scanner::Scanner;
use crate::visitor::Visitor;
use std::marker::PhantomData;
//------------------------------------------------------------------------------
// Peekable
//------------------------------------------------------------------------------

/// A type that can be peeked at the current position of a `Scanner`.
///
/// A `Peekable` is a type that can be used to peek at the current position of a
/// `Scanner`. Implementors of `Peekable` must provide a `peek` method that
/// attempts to match the `Peekable` against the current position of the
/// `Scanner`.
///
/// # Required Methods
///
/// * `peek` - Attempts to match the `Peekable` against the current position of
///   the `Scanner`.
pub trait Peekable<'a, T> {
    /// Attempt to match the `Peekable` against the current position of the
    /// `Scanner`.
    ///
    /// This method will temporarily advance the position of the `Scanner` to
    /// find a match. If a match is found, the `Scanner` is rewound to the
    /// original position and a `PeekResult` is returned. If no match is found,
    /// the `Scanner` is rewound to the original position and an `Err` is
    /// returned.
    ///
    /// # Arguments
    ///
    /// * `data` - The `Scanner` to use when matching.
    ///
    /// # Returns
    ///
    /// A `PeekResult` if the `Peekable` matches the current position of the
    /// `Scanner`, or an `Err` otherwise.
    fn peek(&self, data: &Scanner<'a, T>) -> ParseResult<PeekResult>;
}

//------------------------------------------------------------------------------
// Peeking
//------------------------------------------------------------------------------

/// A successful peeking result.
///
/// A `Peeking` contains the start and end results of a successful peek, the
/// length of the end slice, and a reference to the data that was peeked.
#[derive(Debug, PartialEq)]
pub struct Peeking<'a, T> {
    /// The start of the match.
    pub start_element_size: usize,
    /// The end of the match.
    pub end_element_size: usize,
    /// The length of peeked slice.
    pub end_slice: usize,
    /// The data that was peeked.
    pub data: &'a [T],
}

impl<'a, T> Peeking<'a, T> {
    /// Get a slice of the data that was peeked.
    pub fn peeked_slice(&self) -> &'a [T] {
        &self.data[self.start_element_size..self.end_slice - self.end_element_size]
    }
}

//------------------------------------------------------------------------------
// PeekResult
//------------------------------------------------------------------------------

/// The result of a peeking operation.
///
/// A `PeekResult` contains the result of attempting to match a `Peekable`
/// against the current position of a `Scanner`. If the match succeeds, a
/// `Found` is returned with the length of the end slice, the start of the
/// match, and the end of the match. If the match fails, a `NotFound` is
/// returned.
#[derive(PartialEq, Debug)]
pub enum PeekResult {
    /// The match was successful.
    Found {
        // The last index of the end slice
        end_slice: usize,
        // The size of the start element
        start_element_size: usize,
        // The size of the end element
        end_element_size: usize,
    },
    /// The match was unsuccessful.
    NotFound,
}

//------------------------------------------------------------------------------
// PeekSize
//------------------------------------------------------------------------------

/// A trait that can be used to define a peek size.
pub trait PeekSize {
    /// The `peek_size` method should return the size of the `Peekable`.
    fn peek_size(&self) -> usize;
}

//------------------------------------------------------------------------------
// peek function
//------------------------------------------------------------------------------

/// Attempt to match a `Peekable` against the current position of a `Scanner`.
///
/// This function will temporarily advance the position of the `Scanner` to find
/// a match. If a match is found, the `Scanner` is rewound to the original
/// position and a `Peeking` is returned. If no match is found, the `Scanner` is
/// rewound to the original position and an `Err` is returned.
///
/// # Arguments
///
/// * `peekable` - The `Peekable` to attempt to match.
/// * `scanner` - The `Scanner` to use when matching.
///
/// # Returns
///
/// A `Peeking` if the `Peekable` matches the current position of the `Scanner`,
/// or an `Err` otherwise.
pub fn peek<'a, T, P: Peekable<'a, T>>(
    peekable: P,
    scanner: &Scanner<'a, T>,
) -> ParseResult<Option<Peeking<'a, T>>> {
    let source_cursor = scanner.current_position();
    match peekable.peek(scanner)? {
        PeekResult::Found {
            end_slice,
            start_element_size: start,
            end_element_size: end,
        } => {
            let data = &scanner.data()[source_cursor..source_cursor + end_slice];
            Ok(Some(Peeking {
                start_element_size: start,
                end_element_size: end,
                end_slice,
                data,
            }))
        }
        PeekResult::NotFound => Ok(None),
    }
}

/// Make Peekable any Visitor implementing the PeekSize trait
impl<'a, T, V: Visitor<'a, T> + Clone + PeekSize> Peekable<'a, T> for V {
    fn peek(&self, data: &Scanner<'a, T>) -> ParseResult<PeekResult> {
        // create a temporary scanner to peek data
        let remaining = &data.data()[data.current_position()..];
        let mut scanner = Scanner::new(remaining);
        while !scanner.is_empty() {
            match V::accept(&mut scanner) {
                Ok(element) => {
                    return Ok(PeekResult::Found {
                        end_slice: scanner.current_position(),
                        start_element_size: 0,
                        end_element_size: element.peek_size(),
                    });
                }
                Err(_err) => {
                    scanner.bump_by(1);
                    continue;
                }
            }
        }
        Ok(PeekResult::NotFound)
    }
}

//------------------------------------------------------------------------------
// Until implementations
//------------------------------------------------------------------------------

/// A `Peekable` that peeks until the given `element` is found in the
/// `Scanner`.
///
/// This `Peekable` will temporarily advance the position of the `Scanner` to
/// find a match. If a match is found, the `Scanner` is rewound to the original
/// position and a `PeekResult` is returned. If no match is found, the `Scanner`
/// is rewound to the original position and an `Err` is returned.
#[derive(Clone)]
pub struct Until<'a, T, V> {
    element: V,
    _marker: PhantomData<&'a T>,
}

/// Construct a new `Until`
impl<'a, T, V> Until<'a, T, V> {
    pub fn new(element: V) -> Until<'a, T, V> {
        Until {
            element,
            _marker: PhantomData,
        }
    }
}

/// Implement PeekSize for Until
impl<'a, T, V> PeekSize for Until<'a, T, V>
where
    V: Visitor<'a, T> + Clone + Match<T>,
{
    fn peek_size(&self) -> usize {
        self.element.size()
    }
}

/// Implement Visitor for Until
impl<'a, T, V: Visitor<'a, T>> Visitor<'a, T> for Until<'a, T, V> {
    fn accept(scanner: &mut Scanner<'a, T>) -> ParseResult<Self> {
        Ok(Until::new(V::accept(scanner)?))
    }
}

//------------------------------------------------------------------------------
// UntilEnd implementations
//------------------------------------------------------------------------------

/// A `Peekable` that peeks until the end of the `Scanner`.
///
/// This `Peekable` will temporarily advance the position of the `Scanner` to
/// find a match. If a match is found, the `Scanner` is rewound to the original
/// position and a `PeekResult` is returned. If no match is found, the `Scanner`
/// is rewound to the original position and an `Err` is returned.
#[derive(Default)]
pub struct UntilEnd<T>(PhantomData<T>);

#[cfg(test)]
mod tests {
    use crate::bytes::token::Token;
    use crate::peek::{peek, Until, UntilEnd};

    #[test]
    fn test_until() {
        let data = b"abc|fdgf";
        let mut scanner = crate::scanner::Scanner::new(data);
        let token = Until::new(Token::Pipe);
        let peeked = peek(token, &mut scanner)
            .expect("failed to parse")
            .expect("failed to peek");
        assert_eq!(peeked.peeked_slice(), "abc".as_bytes());
    }

    #[test]
    fn test_until_end() {
        let data = b"abc|fdgf";
        let mut scanner = crate::scanner::Scanner::new(data);
        let token = UntilEnd::default();
        let peeked = peek(token, &mut scanner)
            .expect("failed to parse")
            .expect("failed to peek");
        assert_eq!(peeked.peeked_slice(), "abc|fdgf".as_bytes());
    }
}
