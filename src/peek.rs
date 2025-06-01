//! Peekable types
//!
//! A `Peekable` is a type that can be used to peek at the current position of a
//! `Scanner` without advancing the scanner.

use crate::errors::{ParseError, ParseResult};
use crate::matcher::Match;
use crate::scanner::Scanner;
use crate::visitor::Visitor;
use std::fmt::Debug;
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

impl<'a, T> From<Option<Peeking<'a, T>>> for PeekResult {
    fn from(peeking: Option<Peeking<'a, T>>) -> PeekResult {
        match peeking {
            None => PeekResult::NotFound,
            Some(peeking) => PeekResult::Found {
                end_slice: peeking.end_slice,
                start_element_size: peeking.start_element_size,
                end_element_size: peeking.end_element_size,
            },
        }
    }
}

//------------------------------------------------------------------------------
// PeekSize
//------------------------------------------------------------------------------

/// A trait that can be used to define a peek size.
pub trait PeekSize<T> {
    /// The `peek_size` method should return the size of the `Peekable`.
    fn peek_size(&self) -> usize {
        0
    }
}

/// A default implementation of the `PeekSize` trait for any `Match`.
impl<T, M: Match<T>> PeekSize<T> for M {
    fn peek_size(&self) -> usize {
        self.size()
    }
}

//------------------------------------------------------------------------------
// PeekableImpl
//------------------------------------------------------------------------------

/// Marker for the default implementation of Peekable trait based on Visitor
pub struct DefaultPeekableImplementation;

/// Marker for the customized implementation of Peekable trait user defined
pub struct CustomizedPeekableImplementation;

/// Defines how Peekable should be implemented
///
/// [DefaultPeekableImplementation] will use the default implementation
/// [CustomizedPeekableImplementation] will use the implementation defined by the user
pub trait PeekableImplementation {
    type Type;
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
impl<'a, T, V> Peekable<'a, T> for V
where
    V: Visitor<'a, T> + PeekSize<T> + PeekableImplementation<Type = DefaultPeekableImplementation>,
{
    fn peek(&self, data: &Scanner<'a, T>) -> ParseResult<PeekResult> {
        // create a temporary scanner to peek data
        let mut scanner = Scanner::new(data.remaining());
        while !scanner.is_empty() {
            match V::accept(&mut scanner) {
                Ok(element) => {
                    return Ok(PeekResult::Found {
                        end_slice: scanner.current_position(),
                        start_element_size: 0,
                        end_element_size: element.peek_size(),
                    });
                }
                Err(ParseError::UnexpectedToken) => {
                    return Err(ParseError::UnexpectedToken);
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

//------------------------------------------------------------------------------
// Last implementation
//------------------------------------------------------------------------------

#[derive(Clone)]
pub struct Last<'a, T, V> {
    pub element: V,
    _marker: PhantomData<&'a T>,
}

/// Construct a new `Last`
impl<'a, T, V: Peekable<'a, T>> Last<'a, T, V> {
    pub fn new(element: V) -> Last<'a, T, V> {
        Last {
            element,
            _marker: PhantomData,
        }
    }
}

/// Implement Peekable for Last for all elements implementing Peekable
///
/// Because Last doesn't implement PeekableImplementation<Type = DefaultPeekableImplementation>>
/// there is no conflict
impl<'a, T, V: Peekable<'a, T>> Peekable<'a, T> for Last<'a, T, V> {
    fn peek(&self, scanner: &Scanner<'a, T>) -> ParseResult<PeekResult> {
        let mut state = PeekResult::NotFound;
        let mut inner_scanner = Scanner::new(scanner.remaining());
        let mut positions = vec![];
        // Loop until the scanner is empty
        loop {
            // If the scanner is empty, break
            if inner_scanner.is_empty() {
                break;
            }
            // Peek the element
            let peeked = self.element.peek(&inner_scanner);

            let peeked = match peeked {
                Ok(peeked) => peeked,
                Err(ParseError::UnexpectedToken) => {
                    inner_scanner.bump_by(1);
                    continue;
                }
                Err(err) => {
                    return Err(err);
                }
            };

            // If the pattern was found, add the end slice to the positions
            // and advance the scanner by the end slice
            if let PeekResult::Found { end_slice, .. } = &peeked {
                positions.push(*end_slice);
                inner_scanner.bump_by(*end_slice);
                state = peeked;
            } else {
                if PeekResult::NotFound == state {
                    // The pattern was not found
                    return Ok(PeekResult::NotFound);
                }
                break;
            }
        }

        // Recalculate the end slice from relative positions
        if let PeekResult::Found { end_slice, .. } = &mut state {
            // If there are no positions, the pattern was not found
            if positions.is_empty() {
                return Ok(PeekResult::NotFound);
            }

            // If there is only one position, it is the end slice
            if positions.len() == 1 {
                *end_slice = positions[0];
                return Ok(state);
            }

            // If there are multiple positions, the end slice is the sum of the
            // previous end slices
            let previous_end_slice = &positions[..positions.len() - 1].iter().sum();
            *end_slice += previous_end_slice;
        }

        Ok(state)
    }
}

#[cfg(test)]
mod tests {
    use crate::bytes::token::Token;
    use crate::peek::{peek, Last, UntilEnd};

    #[test]
    fn test_until() {
        let data = b"abc|fdgf";
        let mut scanner = crate::scanner::Scanner::new(data);
        let peeked = peek(Token::Pipe, &mut scanner)
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

    #[test]
    fn test_last() {
        let data = b"abc|def|ghi|";
        let mut scanner = crate::scanner::Scanner::new(data);
        let token = Last::new(Token::Pipe);
        let peeked = peek(token, &mut scanner)
            .expect("failed to parse")
            .expect("failed to peek");
        assert_eq!(peeked.peeked_slice(), "abc|def|ghi".as_bytes());

        let data = b"abc|def|";
        let mut scanner = crate::scanner::Scanner::new(data);
        let token = Last::new(Token::Pipe);
        let peeked = peek(token, &mut scanner)
            .expect("failed to parse")
            .expect("failed to peek");
        assert_eq!(peeked.peeked_slice(), "abc|def".as_bytes());

        let data = b"abc|";
        let mut scanner = crate::scanner::Scanner::new(data);
        let token = Last::new(Token::Pipe);
        let peeked = peek(token, &mut scanner)
            .expect("failed to parse")
            .expect("failed to peek");
        assert_eq!(peeked.peeked_slice(), "abc".as_bytes());

        let data = b"abc";
        let mut scanner = crate::scanner::Scanner::new(data);
        let token = Last::new(Token::Pipe);
        let peeked = peek(token, &mut scanner).expect("failed to parse");
        assert_eq!(peeked, None);
    }
}
