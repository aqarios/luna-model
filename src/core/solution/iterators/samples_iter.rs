use crate::core::expression::BiasConstraints;
use crate::core::solution::AssignmentBaseTypes;
use crate::core::{RcSolution, ResultView, Sample};
use either::Left;

/// Iterates over the sample rows of a solution
#[derive(Debug, Clone)]
pub struct SamplesIterator<Bias, AssignmentTypes>
where
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes,
{
    /// The solution this result view corresponds to
    sol: RcSolution<Bias, AssignmentTypes>,
    /// Index of the next row of the sample within the solution
    next_row: usize,
}

impl<Bias, AssignmentTypes> SamplesIterator<Bias, AssignmentTypes>
where
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes,
{
    pub fn new(sol: RcSolution<Bias, AssignmentTypes>) -> Self {
        Self { sol, next_row: 0 }
    }
}

impl<Bias, AssignmentTypes> Iterator for SamplesIterator<Bias, AssignmentTypes>
where
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes,
{
    type Item = Sample<Bias, AssignmentTypes>;

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