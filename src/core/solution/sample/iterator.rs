use crate::core::{solution::sol::Solution, ValueByIndex};

use super::{Sample, SampleView, VarAssignment};

/// Iterates over the sample rows of a solution
#[derive(Debug, Clone)]
pub struct SamplesIterator<'a> {
    /// The solution this result view corresponds to
    sol: &'a Solution,
    /// Index of the sample within the solution
    row: usize,
}

impl<'a> SamplesIterator<'a> {
    pub fn new(sol: &'a Solution) -> Self {
        Self { sol, row: 0 }
    }
}

impl<'a> Iterator for SamplesIterator<'a> {
    type Item = Sample<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= self.sol.len() {
            None
        } else {
            let sample = Some(Sample::View(SampleView::new(&self.sol, self.row)));
            self.row += 1;
            sample
        }
    }
}

// ITERATOR SINGLE SAMPLE
pub struct SampleIterator<'a> {
    sample: &'a Sample<'a>,
    idx: usize,
}

impl<'a> SampleIterator<'a> {
    pub fn new(sample: &'a Sample<'a>) -> Self {
        Self { sample, idx: 0 }
    }
}

impl<'a> Iterator for SampleIterator<'a> {
    type Item = VarAssignment;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.sample.len() {
            None
        } else {
            let item = self.sample.value_by_index(self.idx.into());
            self.idx += 1;
            Some(item)
        }
    }
}
