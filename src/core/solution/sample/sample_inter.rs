use std::fmt::{Display, Formatter};

use super::{SampleIterator, VarAssignment};
use crate::{
    core::{
        solution::{result::ResultView, sol::Solution},
        writer::SolutionWriter,
        ValueByIndex,
    },
    types::VarIndex,
};
use derive_more::{Deref, DerefMut};
use hashbrown::HashMap;

#[derive(Debug, Clone, Deref, DerefMut)]
pub struct Sample<'a> {
    pub res: ResultView<'a>,
}

impl<'a> Sample<'a> {
    pub fn new(sol: &'a Solution, row: usize) -> Self {
        Self {
            res: ResultView::new(sol, row),
        }
    }
}

impl<'a> ValueByIndex<VarIndex> for Sample<'a> {
    type Output = VarAssignment;

    fn value_by_index(&self, index: VarIndex) -> Self::Output {
        self.res.value_by_index(index)
    }
}

impl<'a> Sample<'a> {
    pub fn variable_names(&self) -> &[String] {
        &self.sol.variable_names
    }

    /// The length of the sample is equal to the number of
    /// variable assignments or the number of variables in
    /// the solution
    pub fn len(&self) -> usize {
        self.sol.n_samples
    }

    pub fn iter(&'a self) -> SampleIterator<'a> {
        SampleIterator::new(self)
    }

    pub fn index_for_variable_name(&self, varname: &str) -> Option<usize> {
        self.sol.variable_names.iter().position(|e| e == varname)
    }

    pub fn to_map(&self) -> HashMap<String, VarAssignment> {
        self.iter()
            .zip(self.sol.variable_names.iter())
            .map(|(v, s)| (s.clone(), v.clone()))
            .collect()
    }
}

impl<'a> Display for Sample<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = SolutionWriter::new().write_sample(self).to_string();
        f.write_str(&s)
    }
}
