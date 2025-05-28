use crate::errors::ParseResult;
use crate::peek::{PeekResult, Peekable, Peeking};
use crate::scanner::Scanner;

/// A [Peeker] is a type that is used to find the best group to forecast
pub struct Peeker<'a, 'b, T, S, E> {
    scanner: &'b Scanner<'a, T>,
    /// Pool of [Peekable]
    peekables: Vec<Box<dyn Peekable<'a, T, S, E> + 'a>>,
}

impl<'a, 'b, T, S, E> Peeker<'a, 'b, T, S, E> {
    pub fn new(scanner: &'b Scanner<'a, T>) -> Self {
        Self {
            scanner,
            peekables: vec![],
        }
    }
}

impl<'a, T, S, E> Peeker<'a, '_, T, S, E> {
    /// Add new [Peekable] element to the peeking pool
    pub fn add_peekable<F: Peekable<'a, T, S, E> + 'a>(mut self, peekable: F) -> Self {
        self.peekables.push(Box::new(peekable));
        self
    }

    /// Run the [Forecast] pool, find the minimal group
    pub fn peek(self) -> ParseResult<Option<Peeking<'a, T, S, E>>> {
        let mut result = None;
        // on boucle sur les possibilités de prédictions
        for peekable in self.peekables.into_iter() {
            let peek_result = peekable.peek(self.scanner)?;
            // on tente de prédire l'élément
            match peek_result {
                // si l'on a trouvé quelque chose
                PeekResult::Found {
                    start,
                    end,
                    end_slice,
                } => {
                    // on récupère le groupe prédit
                    let remaining = self.scanner.remaining();
                    let data = &remaining[..end_slice];
                    let new_forecast = Peeking {
                        start,
                        end,
                        data,
                        end_slice,
                    };
                    match &result {
                        // si l'on n'a encore rien prédit du tout
                        None => {
                            // le groupe trouvé devient le résultat
                            result = Some(new_forecast);
                        }
                        // s'il y a déjà une prédiction
                        Some(min_forecast) => {
                            // on compare la taille du groupe trouvé par rapport
                            // à celui déjà trouvé
                            if new_forecast.data.len() < min_forecast.data.len() {
                                // il devient alors le nouveau groupe prédit
                                result = Some(new_forecast);
                            }
                        }
                    }
                }
                // si la prédiction échoue, on ne fait rien
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
