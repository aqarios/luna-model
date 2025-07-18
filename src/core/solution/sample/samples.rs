use std::fmt::{Display, Formatter};

use derive_more::{Deref, DerefMut};
use crate::core::{writer::SolutionWriter, Solution};
use super::{Sample, SamplesIterator, VarAssignment};

#[derive(Debug, Clone, Deref, DerefMut)]
pub struct Samples<'a>(pub &'a Solution);

impl<'a> Samples<'a> {
    pub fn get_sample(&self, row_idx: usize) -> Option<Sample> {
        self.get_sample_view(row_idx).map(|x| Sample::View(x))
    }

    pub fn get_assignment(&self, row_idx: usize, col_idx: usize) -> Option<VarAssignment> {
        self.0.get_assignment(row_idx, col_idx)
    }

    pub fn iter(&self) -> SamplesIterator {
        SamplesIterator::new(&self.0)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'a> Display for Samples<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = SolutionWriter::new()
            .write_samples(&self, &self.counts)
            .to_string();
        f.write_str(&s)
    }
}
