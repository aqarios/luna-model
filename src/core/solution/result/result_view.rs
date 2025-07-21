use std::fmt::Display;

use crate::{
    core::{
        solution::{sample::SampleView, sol::Solution},
        writer::SolutionWriter,
        Sample, ValueByIndex, VarAssignment,
    },
    types::{Bias, VarIndex},
};

/// A view into a certain sample of a solution and its corresponding metadata.
#[derive(Debug, Clone)]
pub struct ResultView<'a> {
    /// The solution this result view corresponds to
    pub sol: &'a Solution,
    /// Index of the row of the sample within the solution
    pub idx: usize,
}

impl<'a> ResultView<'a> {
    pub fn new(sol: &'a Solution, idx: usize) -> Self {
        Self { sol, idx }
    }
}

impl<'a> ValueByIndex<VarIndex> for ResultView<'a> {
    type Output = VarAssignment;

    fn value_by_index(&self, index: VarIndex) -> Self::Output {
        self.sol.get_assignment(self.idx, index.into()).unwrap()
    }
}

impl<'a> Display for ResultView<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = SolutionWriter::new()
            .write_sample(&Sample::View(SampleView::new(self.sol, self.idx)))
            .to_string();
        f.write_str(&s)
    }
}

impl<'a> ResultView<'a> {
    pub fn obj_value(&self) -> Option<Bias> {
        self.sol.obj_values.as_ref().map(|o| o[self.idx])
    }

    pub fn raw_energy(&self) -> Option<Bias> {
        self.sol.raw_energies[self.idx]
    }

    pub fn constraint_satisfaction(&self) -> Option<Vec<bool>> {
        self.sol.constraints.as_ref().map(|c| c[self.idx].clone())
    }

    pub fn variable_bounds_satisfaction(&self) -> Option<Vec<bool>> {
        self.sol
            .variable_bounds
            .as_ref()
            .map(|v| v[self.idx].clone())
    }

    pub fn feasible(&self) -> Option<bool> {
        self.sol.feasible.as_ref().map(|f| f[self.idx])
    }
}
