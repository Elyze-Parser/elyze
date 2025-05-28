//! Peekable types
//!
//! A `Peekable` is a type that can be used to peek at the current position of a
//! `Scanner` without advancing the scanner.

use crate::errors::ParseResult;
use crate::matcher::MatchSize;
use crate::recognizer::Recognizable;
use crate::scanner::Scanner;
use std::marker::PhantomData;

/// A successful peeking result.
///
/// A `Peeking` contains the start and end results of a successful peek, the
/// length of the end slice, and a reference to the data that was peeked.
#[derive(Debug, PartialEq)]
pub struct Peeking<'a, T, S, E> {
    /// The start of the match.
    pub start: S,
    /// The end of the match.
    pub end: E,
    /// The length of peeked slice.
    pub end_slice: usize,
    /// The data that was peeked.
    pub data: &'a [T],
}

impl<'a, T, S, E> Peeking<'a, T, S, E>
where
    S: MatchSize,
    E: MatchSize,
{
    /// Get a slice of the data that was peeked.
    pub fn peeked_slice(&self) -> &'a [T] {
        &self.data[self.start.size()..self.end_slice - self.end.size()]
    }

    /// Get the data that was peeked.
    ///
    /// Returns a reference to the underlying data that was peeked.
    pub fn data(&self) -> &'a [T] {
        self.data
    }
}

/// The result of a peeking operation.
///
/// A `PeekResult` contains the result of attempting to match a `Peekable`
/// against the current position of a `Scanner`. If the match succeeds, a
/// `Found` is returned with the length of the end slice, the start of the
/// match, and the end of the match. If the match fails, a `NotFound` is
/// returned.
#[derive(PartialEq, Debug)]
pub enum PeekResult<S, E> {
    /// The match was successful.
    Found { end_slice: usize, start: S, end: E },
    /// The match was unsuccessful.
    NotFound,
}

/// A type that can be peeked at the current position of a `Scanner`.
///
/// A `Peekable` is a type that can be used to peek at the current position of a
/// `Scanner`. Implementors of `Peekable` must provide a `peek` method that
/// attempts to match the `Peekable` against the current position of the
/// `Scanner`.
///
/// # Associated Types
///
/// * `S` - The type of the start of the match.
/// * `E` - The type of the end of the match.
///
/// # Required Methods
///
/// * `peek` - Attempts to match the `Peekable` against the current position of
///   the `Scanner`.
pub trait Peekable<'a, T, S, E> {
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
    fn peek(&self, data: &Scanner<'a, T>) -> ParseResult<PeekResult<S, E>>;
}

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
pub fn peek<'a, T, S, E, P: Peekable<'a, T, S, E>>(
    peekable: P,
    scanner: &mut Scanner<'a, T>,
) -> ParseResult<Option<Peeking<'a, T, S, E>>> {
    let source_cursor = scanner.current_position();
    match peekable.peek(scanner)? {
        PeekResult::Found {
            end_slice,
            start,
            end,
        } => {
            let data = &scanner.data()[source_cursor..source_cursor + end_slice];
            Ok(Some(Peeking {
                start,
                end,
                end_slice,
                data,
            }))
        }
        PeekResult::NotFound => {
            scanner.jump_to(source_cursor);
            Ok(None)
        }
    }
}

/// A `Peekable` that peeks until the given `element` is found in the
/// `Scanner`.
///
/// This `Peekable` will temporarily advance the position of the `Scanner` to
/// find a match. If a match is found, the `Scanner` is rewound to the original
/// position and a `PeekResult` is returned. If no match is found, the `Scanner`
/// is rewound to the original position and an `Err` is returned.
pub struct Until<'a, T, V> {
    element: V,
    _marker: PhantomData<&'a T>,
}

impl<'a, T, V> Until<'a, T, V> {
    pub fn new(element: V) -> Until<'a, T, V> {
        Until {
            element,
            _marker: PhantomData,
        }
    }
}

impl<'a, T, V> Peekable<'a, T, V, V> for Until<'a, T, V>
where
    V: Recognizable<'a, T, V> + Clone,
{
    /// Peek until the given `element` is found in the `Scanner`.
    ///
    /// This function will temporarily advance the position of the `Scanner` to
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
    /// A `PeekResult` if the `element` matches the current position of the
    /// `Scanner`, or an `Err` otherwise.
    fn peek(&self, data: &Scanner<'a, T>) -> ParseResult<PeekResult<V, V>> {
        // create a temporary scanner to peek data
        let remaining = &data.data()[data.current_position()..];
        let mut scanner = Scanner::new(remaining);
        while !scanner.is_empty() {
            match self.element.clone().recognize(&mut scanner) {
                Ok(Some(element)) => {
                    return Ok(PeekResult::Found {
                        end_slice: scanner.current_position() - self.element.size(),
                        start: element.clone(),
                        end: element.clone(),
                    });
                }
                Ok(None) => {
                    scanner.bump_by(1);
                    continue;
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
    use crate::peek::{Until, UntilEnd, peek};

    #[test]
    fn test_until() {
        let data = b"abc|fdgf";
        let mut scanner = crate::scanner::Scanner::new(data);
        let token = Until::new(Token::Pipe);
        let peeked = peek(token, &mut scanner)
            .expect("failed to parse")
            .expect("failed to peek");
        assert_eq!(peeked.data(), "abc".as_bytes());
    }

    #[test]
    fn test_until_end() {
        let data = b"abc|fdgf";
        let mut scanner = crate::scanner::Scanner::new(data);
        let token = UntilEnd::default();
        let peeked = peek(token, &mut scanner)
            .expect("failed to parse")
            .expect("failed to peek");
        assert_eq!(peeked.data, "abc|fdgf".as_bytes());
    }
}
