use crate::core::{RcSolution, ResultView};

/// Iterates over the single results of a solution
#[derive(Debug, Clone)]
pub struct ResultIterator {
    /// The solution this result view corresponds to
    sol: RcSolution,
    /// Index of the next row of the sample within the solution
    next_row: usize,
}

impl ResultIterator {
    pub fn new(sol: RcSolution) -> Self {
        Self { sol, next_row: 0 }
    }
}

impl Iterator for ResultIterator {
    type Item = ResultView;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_row >= self.sol.borrow().len() {
            None
        } else {
            let res_view = Some(ResultView::new(RcSolution::clone(&self.sol), self.next_row));
            self.next_row += 1;
            res_view
        }
    }
}
