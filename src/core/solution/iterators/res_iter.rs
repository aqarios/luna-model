use crate::core::expression::BiasConstraints;
use crate::core::solution::AssignmentBaseTypes;
use crate::core::{RcSolution, ResultView};

/// Iterates over the single results of a solution
#[derive(Debug, Clone)]
pub struct ResultIterator<Bias, AssignmentTypes>
where
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes,
{
    /// The solution this result view corresponds to
    sol: RcSolution<Bias, AssignmentTypes>,
    /// Index of the next row of the sample within the solution
    next_row: usize,
}

impl<Bias, AssignmentTypes> ResultIterator<Bias, AssignmentTypes>
where
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes,
{
    pub fn new(sol: RcSolution<Bias, AssignmentTypes>) -> Self {
        Self { sol, next_row: 0 }
    }
}

impl<Bias, AssignmentTypes> Iterator for ResultIterator<Bias, AssignmentTypes>
where
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes,
{
    type Item = ResultView<Bias, AssignmentTypes>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_row >= self.sol.len() {
            None
        } else {
            let res_view = Some(ResultView::new(RcSolution::clone(&self.sol), self.next_row));
            self.next_row += 1;
            res_view
        }
    }
}
