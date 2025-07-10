use crate::core::{SharedSolution, ResultView, Sample};
use either::Left;

/// Iterates over the sample rows of a solution
#[derive(Debug, Clone)]
pub struct SamplesIterator {
    /// The solution this result view corresponds to
    sol: SharedSolution,
    /// Index of the next row of the sample within the solution
    next_row: usize,
}

impl SamplesIterator {
    pub fn new(sol: SharedSolution) -> Self {
        Self { sol, next_row: 0 }
    }
}

impl Iterator for SamplesIterator {
    type Item = Sample;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_row >= self.sol.borrow().len() {
            None
        } else {
            let sample = Some(Sample(Left(ResultView::new(
                SharedSolution::clone(&self.sol),
                self.next_row,
            ))));
            self.next_row += 1;
            sample
        }
    }
}
