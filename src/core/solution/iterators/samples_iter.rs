use crate::core::{RcSolution, ResultView, Sample};
use either::Left;

/// Iterates over the sample rows of a solution
#[derive(Debug, Clone)]
pub struct SamplesIterator {
    /// The solution this result view corresponds to
    sol: RcSolution,
    /// Index of the next row of the sample within the solution
    next_row: usize,
}

impl SamplesIterator {
    pub fn new(sol: RcSolution) -> Self {
        Self { sol, next_row: 0 }
    }
}

impl Iterator for SamplesIterator {
    type Item = Sample;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_row >= self.sol.len() {
            None
        } else {
            let sample = Some(Sample(Left(ResultView::new(
                RcSolution::clone(&self.sol),
                self.next_row,
            ))));
            self.next_row += 1;
            sample
        }
    }
}
