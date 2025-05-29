use crate::errors::ParseResult;
use crate::peek::{PeekResult, Peekable, Peeking};
use crate::scanner::Scanner;

/// A [Peeker] is a type that is used to find the best group to forecast
pub struct Peeker<'a, 'b, T> {
    scanner: &'b Scanner<'a, T>,
    /// Pool of [Peekable]
    peekables: Vec<Box<dyn Peekable<'a, T> + 'a>>,
}

impl<'a, 'b, T> Peeker<'a, 'b, T> {
    pub fn new(scanner: &'b Scanner<'a, T>) -> Self {
        Self {
            scanner,
            peekables: vec![],
        }
    }
}

impl<'a, T> Peeker<'a, '_, T> {
    /// Add new [Peekable] element to the peeking pool
    pub fn add_peekable<F: Peekable<'a, T> + 'a>(mut self, peekable: F) -> Self {
        self.peekables.push(Box::new(peekable));
        self
    }

    /// Run the [Forecast] pool, find the minimal group
    pub fn peek(self) -> ParseResult<Option<Peeking<'a, T>>> {
        let mut result = None;
        // loop on the possibilities of predictions
        for peekable in self.peekables.into_iter() {
            let peek_result = peekable.peek(self.scanner)?;
            // we try to predict the element
            match peek_result {
                // if we have found something
                PeekResult::Found {
                    start_element_size: start,
                    end_element_size: end,
                    end_slice,
                } => {
                    // we get the predicted group
                    let remaining = self.scanner.remaining();
                    let data = &remaining[..end_slice];
                    let new_forecast = Peeking {
                        start_element_size: start,
                        end_element_size: end,
                        data,
                        end_slice,
                    };
                    match &result {
                        // if we have not predicted anything yet
                        None => {
                            // the group found becomes the result
                            result = Some(new_forecast);
                        }
                        // if there is already a prediction
                        Some(min_forecast) => {
                            // we compare the size of the group found with the
                            // one already found
                            if new_forecast.data.len() < min_forecast.data.len() {
                                // it becomes the new predicted group
                                result = Some(new_forecast);
                            }
                        }
                    }
                }
                // if the prediction fails, we do nothing
                PeekResult::NotFound => {}
            }
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::bytes::token::Token;
    use crate::peek::{Until, UntilEnd};
    use crate::peeker::Peeker;
    use crate::scanner::Scanner;

    #[test]
    fn test_peeker() {
        let data = b"data\n";
        let scanner = Scanner::new(data);
        let peeker = Peeker::new(&scanner)
            .add_peekable(Until::new(Token::Ln))
            .add_peekable(UntilEnd::default());
        let result = peeker
            .peek()
            .expect("failed to parse")
            .expect("failed to peek");
        assert_eq!(result.data, "data".as_bytes());

        let data = b"data";
        let scanner = Scanner::new(data);
        let peeker = Peeker::new(&scanner)
            .add_peekable(Until::new(Token::Ln))
            .add_peekable(UntilEnd::default());
        let result = peeker
            .peek()
            .expect("failed to parse")
            .expect("failed to peek");
        assert_eq!(result.data, "data".as_bytes());
    }
}
