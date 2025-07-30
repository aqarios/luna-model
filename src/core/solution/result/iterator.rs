use super::ResultView;
use crate::core::solution::sol::Solution;

/// Iterates over the sample rows of a solution
#[derive(Debug, Clone)]
pub struct ResultViewsIterator<'a> {
    /// The solution this result view corresponds to
    sol: &'a Solution,
    /// Index of the sample within the solution
    row: usize,
}

impl<'a> ResultViewsIterator<'a> {
    pub fn new(sol: &'a Solution) -> Self {
        Self { sol, row: 0 }
    }
}

impl<'a> Iterator for ResultViewsIterator<'a> {
    type Item = ResultView<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= self.sol.len() {
            None
        } else {
            let sample = Some(ResultView::new(&self.sol, self.row));
            self.row += 1;
            sample
        }
    }
}
