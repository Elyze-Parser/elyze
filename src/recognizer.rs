use crate::errors::ParseResult;
use crate::scanner::Scanner;

/// Describes a recognizable object.
pub trait Recognizable<'a, V>: Sized {
    /// Try to recognize the object for the given scanner.
    ///
    /// # Type Parameters
    /// V - The type of the object to recognize
    ///
    /// # Arguments
    /// * `scanner` - The scanner to recognize the object for.
    ///
    /// # Returns
    /// * `Ok(Some(V))` if the object was recognized,
    /// * `Ok(None)` if the object was not recognized,
    /// * `Err(ParseError)` if an error occurred
    ///
    fn recognize(scanner: &mut Scanner<'a, V>) -> ParseResult<Option<V>>;
}
