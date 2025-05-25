use crate::errors::{ParseError, ParseResult};
use crate::matcher::Match;
use crate::scanner::Scanner;

pub mod matchers;
pub mod token;

/// A scanner that works on bytes.
pub type Tokenizer<'a> = Scanner<'a, u8>;

impl Tokenizer<'_> {
    /// Run a matcher on the tokenizer.
    ///
    /// # Arguments
    ///
    /// * `matcher` - The matcher to run.
    ///
    /// # Returns
    ///
    /// The result of running the matcher on the tokenizer.
    ///
    /// # Errors
    ///
    /// If the tokenizer is empty, returns `ParseError::UnexpectedEndOfInput`.
    pub fn recognize(&mut self, matcher: impl Match<u8>) -> ParseResult<(bool, usize)> {
        if self.is_empty() {
            return Err(ParseError::UnexpectedEndOfInput);
        }

        let data = self.remaining();
        Ok(matcher.matcher(data))
    }
}
