use super::sol::Solution;
use crate::core::{
    solution::{result::ResultView, sample::SamplesIterator},
    VarAssignment,
};

impl Solution {
    pub fn len(&self) -> usize {
        self.n_samples
    }

    pub fn best(&self) -> Option<ResultView> {
        self.best_sample_idx.map(|idx| ResultView::new(self, idx))
    }

    pub fn get_result_view(&self, idx: usize) -> Option<ResultView> {
        if idx >= self.n_samples {
            None
        } else {
            Some(ResultView::new(self, idx))
        }
    }

    pub fn get_assignment(&self, row: usize, col: usize) -> Option<VarAssignment> {
        self.samples.get(col).and_then(|column| column.get(row))
    }

    pub fn iter_samples<'a>(&'a self) -> SamplesIterator<'a> {
        SamplesIterator::new(self)
    }
}
