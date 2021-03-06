use crate::IMExIterator;

/// An IMExIterator that keeps track of how many times it has iterated with some result.
/// Used by QuantifiedIMExVal to keep track of inner IMExVal iterations.
#[derive(PartialEq, Debug, Clone)]
pub struct IMExIterCounter<X: IMExIterator> {
    imex_iter: X,
    counter: u32,
}

impl<X: IMExIterator> IMExIterator for IMExIterCounter<X> {
    fn iterate<T, I>(&mut self, iters: &mut Vec<T>) -> Option<I>
    where
        T: Iterator<Item = I>,
    {
        match self.imex_iter.iterate(iters) {
            Some(res) => {
                self.counter += 1;
                Some(res)
            }
            None => None,
        }
    }
}

impl<X: IMExIterator> IMExIterCounter<X> {
    /// Construct a new IMExIterCounter from any IMExIterator
    pub fn new(imex_iter: X) -> IMExIterCounter<X> {
        IMExIterCounter {
            imex_iter,
            counter: 0,
        }
    }

    /// Get the current number of times that this IMExIterCounter was iterated with some result.
    pub fn count(&self) -> u32 {
        self.counter
    }
}

use nom::{error::VerboseError, IResult};

/// Trait for implementing parser-combinator-style parse functions
pub trait ParserCombinator {
    /// Defines how an object is parsed from a string.
    ///
    /// Returns a nom::IResult containing the Self that was parsed and the remainder of the input
    /// string that wasn't parsed.
    fn parse(input: &str) -> IResult<&str, Self, VerboseError<&str>>
    where
        Self: std::marker::Sized;
}

#[cfg(test)]
mod iter_counter_tests {
    use super::super::IMEx;
    use super::*;
    use std::{convert::TryFrom, io::Result};

    #[test]
    fn iter_counter_counts_iterations() -> Result<()> {
        let mut iter_counter = IMExIterCounter::new(IMEx::try_from("0*")?);
        let mut iters = vec!["123".chars()];

        assert_eq!(iter_counter.count(), 0);

        iter_counter.iterate(&mut iters);
        assert_eq!(iter_counter.count(), 1);

        iter_counter.iterate(&mut iters);
        assert_eq!(iter_counter.count(), 2);

        iter_counter.iterate(&mut iters);
        assert_eq!(iter_counter.count(), 3);

        iter_counter.iterate(&mut iters);
        assert_eq!(iter_counter.count(), 3);

        Ok(())
    }

    #[test]
    fn iter_counter_wont_count_past_zero_on_empty_imex() -> Result<()> {
        let mut iter_counter = IMExIterCounter::new(IMEx::try_from("")?);
        let mut iters = vec!["123".chars()];

        assert_eq!(iter_counter.count(), 0);

        iter_counter.iterate(&mut iters);
        assert_eq!(iter_counter.count(), 0);

        iter_counter.iterate(&mut iters);
        assert_eq!(iter_counter.count(), 0);

        Ok(())
    }
}
