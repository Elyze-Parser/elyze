//! Group components

use crate::bytes::token::Token;
use crate::errors::ParseResult;
use crate::matcher::Match;
use crate::peek::{peek, PeekResult, Peekable};
use crate::recognizer::Recognizable;
use crate::scanner::Scanner;

/// Checks if the current token is escaped by looking for an escape token before it.
///
/// Rewinds the scanner by the size of the escape token and checks if the escape token
/// is present at that position. Returns true if the escape token is found, false otherwise.
///
/// # Arguments
///
/// * `scanner` - Scanner positioned at the token to check
/// * `escape_token` - The escape token to look for
///
/// # Type Parameters
///
/// * `T` - The type of the escape token
/// * `V` - The return type of the escape token's recognize() method
///
/// # Returns
///
/// * `Ok(true)` if the token is escaped (escape token found before it)
/// * `Ok(false)` if the token is not escaped
/// * `Err(ParseError)` if scanning fails
fn is_escaped<'a, T, V>(mut scanner: Scanner<'a, u8>, escape_token: T) -> ParseResult<bool>
where
    T: Recognizable<'a, u8, V> + Copy,
{
    // If the current position is less than the size of the escape token, the token is not escaped
    if scanner.current_position() < escape_token.size() {
        return Ok(false);
    }

    scanner.rewind(escape_token.size());
    // Try to recognize the escape token
    if escape_token.recognize(&mut scanner)?.is_some() {
        // If it is present, the token is escaped
        return Ok(true);
    }
    Ok(false)
}

/// Try to recognize either a start group or an end group token.
///
/// If the start group token is recognized, increment the balancing counter.
/// If the end group token is recognized, decrement the balancing counter.
/// If neither is recognized, move the tokenizer by one byte.
///
/// # Arguments
///
/// * `tokenizer` - The tokenizer to use
/// * `balance` - A mutable reference to the balancing counter
/// * `start` - The start group token to recognize
/// * `end` - The end group token to recognize
///
/// # Errors
///
/// Returns `Err(ParseError)` if the tokenizer encounters an error.
///
/// # Examples
///
///
pub fn match_for_balanced_group<'a, T1, T2, T3, V3>(
    scanner: &mut Scanner<'a, u8>,
    balance: &mut usize,
    start: T1,
    end: T2,
    escape_token: T3,
) -> ParseResult<()>
where
    T1: Peekable<'a, u8> + Match<u8> + Copy,
    T2: Peekable<'a, u8> + Match<u8> + Copy,
    T3: Recognizable<'a, u8, V3> + Copy,
{
    match peek(start, scanner)? {
        Some(peeking) => {
            scanner.bump_by(peeking.end_slice);
            let mut rewind_scanner = scanner.clone();
            rewind_scanner.rewind(start.size());
            // if start group token increment balancing counter
            if !is_escaped(rewind_scanner, escape_token)? {
                *balance += 1
            }
            return Ok(());
        }
        // it's not a start token
        None => {}
    }

    match peek(end, scanner)? {
        // if end group token decrement balancing counter
        Some(peeking) => {
            scanner.bump_by(peeking.end_slice);
            let mut rewind_scanner = scanner.clone();
            rewind_scanner.rewind(end.size());
            if is_escaped(rewind_scanner, escape_token)? {
                return Ok(());
            }

            *balance -= 1;
        }
        // if neither, move by one byte
        None => {
            scanner.bump_by(1);
            return Ok(());
        }
    }

    Ok(())
}

/// A closure that takes a slice of bytes and returns a `PeekResult` indicating
/// whether the slice matches a balanced group.
///
/// A balanced group is a sequence of bytes that has the same number of start
/// and end group tokens. The start group token is recognized by the `start`
/// parameter and the end group token is recognized by the `end` parameter.
///
/// The closure returns `Ok(PeekResult::Found { end_slice, start, end })` if the
/// slice matches a balanced group, `Ok(PeekResult::NotFound)` if the slice
/// does not match a balanced group, and `Err(ParseError)` if there is an error
/// recognizing the tokens.
///
/// # Arguments
///
/// * `start` - The start group token to recognize
/// * `end` - The end group token to recognize
///
/// # Returns
///
/// A closure that takes a slice of bytes and returns a `PeekResult` indicating
/// whether the slice matches a balanced group.
pub fn match_group<'a, T1, T2, T3, V3>(
    start: T1,
    end: T2,
    escape_token: T3,
) -> impl Fn(&'a [u8]) -> ParseResult<PeekResult> + 'a
where
    T1: Peekable<'a, u8> + Match<u8> + Copy + 'a,
    T2: Peekable<'a, u8> + Match<u8> + Copy + 'a,
    T3: Recognizable<'a, u8, V3> + Copy + 'a,
{
    move |input: &'a [u8]| {
        // 0 if number of start token equals number of end token
        // i.e: ( 5 + 3 - ( 10 * 8 ) ) => 2 "(" and 2 ")" => balanced
        //      ( 5 + 3 - ( 10 * 8 )   => 2 "(" and 1 ")" => unbalanced
        let mut balance = 1;

        let mut scanner = Scanner::new(input);

        if start.recognize(&mut scanner)?.is_none() {
            return Ok(PeekResult::NotFound);
        }

        loop {
            match_for_balanced_group(&mut scanner, &mut balance, start, end, escape_token)?;
            // if balancing is 0 then either there is no group at all or is balanced
            if balance == 0 {
                break;
            }
        }

        // not enough bytes to create a group
        if scanner.current_position() == 1 {
            return Ok(PeekResult::NotFound);
        }

        Ok(PeekResult::Found {
            end_slice: scanner.current_position(),
            start_element_size: start.size(),
            end_element_size: end.size(),
        })
    }
}

/// A closure that takes a slice of bytes and returns a `PeekResult` indicating
/// whether the slice matches a delimited group.
///
/// A delimited group is a sequence of bytes that starts and ends with the same
/// token and has no other occurrence of that token in between. The token is
/// recognized by the `token` parameter and the escape token is recognized by the
/// `escape_token` parameter.
///
/// The closure returns `Ok(PeekResult::Found { end_slice, start, end })` if the
/// slice matches a delimited group, `Ok(PeekResult::NotFound)` if the slice
/// does not match a delimited group, and `Err(ParseError)` if there is an error
/// recognizing the tokens.
///
/// # Arguments
///
/// * `token` - The token to recognize at the start and end of the group
/// * `escape_token` - The escape token to recognize and ignore in the group
///
/// # Returns
///
/// A closure that takes a slice of bytes and returns a `PeekResult` indicating
/// whether the slice matches a delimited group.
pub fn match_for_delimited_group<'a, T, T2>(
    token: T,
    escape_token: T2,
) -> impl Fn(&'a [u8]) -> ParseResult<PeekResult> + 'a
where
    T: Peekable<'a, u8> + Copy + 'a + Match<u8>,
    T2: Peekable<'a, u8> + Copy + 'a + Match<u8>,
{
    move |input: &'a [u8]| {
        // The group must be at least two tokens long
        if input.len() < token.size() * 2 {
            return Ok(PeekResult::NotFound);
        }

        // Create a scanner from the input
        let mut scanner = Scanner::new(input);

        // The group must start with the token
        if token.recognize(&mut scanner)?.is_none() {
            return Ok(PeekResult::NotFound);
        }

        // This flag indicates whether the prediction was successful
        let mut found = false;
        // While there are still bytes in the input
        while !scanner.remaining().is_empty() {
            // If the token is recognized somewhere in the input
            match peek(token, &mut scanner)? {
                Some(peeking) => {
                    scanner.bump_by(peeking.end_slice);
                    let mut rewind_scanner = scanner.clone();
                    rewind_scanner.rewind(token.size());
                    // Advance the scanner by the size of the peeked token
                    // If the token is escaped
                    if is_escaped(rewind_scanner, escape_token)? {
                        // Advance the scanner by one byte
                        scanner.bump_by(1);
                        continue;
                    }
                    found = true;
                    break;
                }
                None => break,
            };
        }

        // If the prediction was unsuccessful
        if !found {
            return Ok(PeekResult::NotFound);
        }

        Ok(PeekResult::Found {
            end_slice: scanner.current_position(),
            start_element_size: token.size(),
            end_element_size: token.size(),
        })
    }
}

/// Types of groups
///
/// This enum is used to specify the type of a group in a matcher.
pub enum GroupKind {
    /// A group enclosed in parentheses
    Parenthesis,
    /// A group enclosed in single quotes
    Quotes,
    /// A group enclosed in double quotes
    DoubleQuotes,
}

type GroupMatcher<'a> = Box<dyn Fn(&'a [u8]) -> ParseResult<PeekResult> + 'a>;

impl GroupKind {
    fn matcher<'a>(&self) -> GroupMatcher<'a>
where {
        match self {
            GroupKind::Parenthesis => Box::new(match_group(
                Token::OpenParen,
                Token::CloseParen,
                Token::Backslash,
            )),
            GroupKind::Quotes => {
                Box::new(match_for_delimited_group(Token::Quote, Token::Backslash))
            }
            GroupKind::DoubleQuotes => Box::new(match_for_delimited_group(
                Token::DoubleQuote,
                Token::Backslash,
            )),
        }
    }
}

impl<'a> Peekable<'a, u8> for GroupKind {
    fn peek(&self, data: &Scanner<'a, u8>) -> ParseResult<PeekResult> {
        self.matcher()(data.remaining())
    }
}

#[cfg(test)]
mod tests {
    use crate::bytes::components::groups::{match_for_delimited_group, match_group, GroupKind};
    use crate::bytes::token::Token;
    use crate::errors::ParseResult;
    use crate::peek::{peek, PeekResult, Peeking};
    use crate::scanner::Scanner;

    #[test]
    fn test_match_group() {
        let data = "( 5 + 3 - ( 10 * 8 ) \\)) + 54";
        let result =
            match_group(Token::OpenParen, Token::CloseParen, Token::Backslash)(data.as_bytes())
                .expect("failed to parse");
        assert_eq!(
            result,
            PeekResult::Found {
                end_slice: 24,
                start_element_size: 1,
                end_element_size: 1
            }
        );
        assert_eq!(&data[..24].as_bytes(), b"( 5 + 3 - ( 10 * 8 ) \\))");
    }

    #[test]
    fn test_match_group2() -> ParseResult<()> {
        let data = "( 5 + 3 - \\( ( 10 * 8 \\)) \\)) + 54";
        let mut tokenizer = Scanner::new(data.as_bytes());
        let result = peek(GroupKind::Parenthesis, &mut tokenizer)?;

        if let Some(peeked) = result {
            assert_eq!(peeked.peeked_slice(), b" 5 + 3 - \\( ( 10 * 8 \\)) \\)");
        }
        Ok(())
    }

    #[test]
    fn test_non_match_group() {
        let data = "4 + ( 5 + 3 - ( 10 * 8 ) \\)) + 54";
        let result =
            match_group(Token::OpenParen, Token::CloseParen, Token::Backslash)(data.as_bytes())
                .expect("failed to parse");
        assert_eq!(result, PeekResult::NotFound);
    }

    #[test]
    fn test_match_group_delimited() {
        let data = b"( 5 + 3 - ( 10 * 8 ) ) + 54";
        let mut tokenizer = Scanner::new(data);
        let result = peek(GroupKind::Parenthesis, &mut tokenizer).expect("failed to parse");
        assert_eq!(
            result,
            Some(Peeking {
                start_element_size: 1,
                end_element_size: 1,
                data: &data[0..22],
                end_slice: 22
            })
        );
        assert_eq!(&data[..22], b"( 5 + 3 - ( 10 * 8 ) )");
    }

    #[test]
    fn test_match_group_delimited2() {
        let data = b"( 5 + 3 - ( 10 * 8 ) ) + 54";
        let mut tokenizer = Scanner::new(data);
        let result = peek(GroupKind::Parenthesis, &mut tokenizer).expect("failed to parse");

        if let Some(peeked) = result {
            assert_eq!(peeked.peeked_slice(), b" 5 + 3 - ( 10 * 8 ) ");
        }
    }

    #[test]
    fn test_match_quotes2() {
        let data = b"'hello world' data";
        let mut tokenizer = Scanner::new(data);
        let result = peek(GroupKind::Quotes, &mut tokenizer).expect("failed to parse");

        if let Some(peeked) = result {
            assert_eq!(peeked.peeked_slice(), b"hello world");
        }
    }

    #[test]
    fn test_match_quotes3() {
        let data = "'I\\'m a quoted data' - 'yes me too'";
        let mut tokenizer = Scanner::new(data.as_bytes());
        let result = peek(GroupKind::Quotes, &mut tokenizer).expect("failed to parse");

        if let Some(peeked) = result {
            assert_eq!(peeked.peeked_slice(), b"I\\'m a quoted data");
        }
    }

    #[test]
    fn test_match_quotes() {
        let data = b"'hello world' data";
        let result = match_for_delimited_group(Token::Quote, Token::Backslash)(data)
            .expect("failed to parse");
        assert_eq!(
            result,
            PeekResult::Found {
                end_slice: 13,
                start_element_size: 1,
                end_element_size: 1
            }
        );
        assert_eq!(&data[..13], b"'hello world'");

        let data = r#"'hello world l\'éléphant' data"#;
        let result = match_for_delimited_group(Token::Quote, Token::Backslash)(data.as_bytes())
            .expect("failed to parse");
        assert_eq!(
            result,
            PeekResult::Found {
                end_slice: 27,
                start_element_size: 1,
                end_element_size: 1
            }
        );
        assert_eq!(&data[..27], r#"'hello world l\'éléphant'"#);

        let data = "\"hello world\" data";
        let result =
            match_for_delimited_group(Token::DoubleQuote, Token::Backslash)(data.as_bytes())
                .expect("failed to parse");
        assert_eq!(
            result,
            PeekResult::Found {
                end_slice: 13,
                start_element_size: 1,
                end_element_size: 1
            }
        );
        assert_eq!(&data[..13], "\"hello world\"");

        let data = r#""hello world" data"#;
        let result =
            match_for_delimited_group(Token::DoubleQuote, Token::Backslash)(data.as_bytes())
                .expect("failed to parse");
        assert_eq!(
            result,
            PeekResult::Found {
                end_slice: 13,
                start_element_size: 1,
                end_element_size: 1
            }
        );
        assert_eq!(&data[..13], r#""hello world""#);
    }
}
