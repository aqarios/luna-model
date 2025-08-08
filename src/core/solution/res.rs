use crate::core::solution::sol::VarAssignment;
use crate::core::writer::SolutionWriter;
use crate::core::{Sample, SampleIterator, SharedSolution, ValueByIndex};
use crate::types::{Bias, VarIndex};
use either::{Left, Right};
use std::fmt::{Display, Formatter};

use super::sample::OwnedSample;

/// A view into a certain sample of a solution and its corresponding metadata.
#[derive(Debug, Clone)]
pub struct ResultView {
    /// The solution this result view corresponds to
    pub sol: SharedSolution,
    /// Index of the row of the sample within the solution
    pub row_idx: usize,
}

impl ResultView {
    pub fn new(sol: SharedSolution, row_idx: usize) -> Self {
        Self { sol, row_idx }
    }

    pub fn iter(&self) -> SampleIterator {
        SampleIterator::from_res_view(&self)
    }

    pub fn obj_value(&self) -> Option<Bias> {
        self.sol.access().obj_values[self.row_idx]
    }

    pub fn raw_energy(&self) -> Option<Bias> {
        self.sol.access().raw_energies[self.row_idx]
    }

    pub fn constraint_satisfaction(&self) -> Option<Vec<bool>> {
        self.sol.access().constraints[self.row_idx].clone()
    }

    pub fn variable_bounds_satisfaction(&self) -> Option<Vec<bool>> {
        self.sol.access().variable_bounds[self.row_idx].clone()
    }

    pub fn feasible(&self) -> Option<bool> {
        self.sol.access().feasible[self.row_idx]
    }

    pub fn counts(&self) -> usize {
        self.sol.access().counts[self.row_idx]
    }

    pub fn get_assignment(&self, col_idx: usize) -> Option<VarAssignment> {
        self.sol.access().get_assignment(self.row_idx, col_idx)
    }

    pub fn get_sample(&self) -> Sample {
        Sample(Left(self.clone()))
    }

    pub fn map_varidx(&self, varidx: usize) -> usize {
        self.sol.access().map_varidx(varidx)
    }
}

impl PartialEq for ResultView {
    fn eq(&self, other: &Self) -> bool {
        self.row_idx == other.row_idx && self.sol == other.sol
    }
}

impl ValueByIndex<VarIndex> for ResultView {
    type Output = VarAssignment;

    fn value_by_index(&self, index: VarIndex) -> Self::Output {
        self.sol
            .access()
            .get_assignment(self.row_idx, self.map_varidx(index.into()))
            .unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct OwnedResult {
    /// The vector of variable assignments.
    pub sample: OwnedSample,
    /// The objective value computed from an AqModel. If not present, a raw value from the solver
    /// may be used. None, if none of these are present.
    pub obj_value: Option<Bias>,
    /// Boolean flag for each single constraint whether it's satisfied.
    pub constraint_satisfaction: Option<Vec<bool>>,
    /// Boolean flag for each variable bounds whether it's satisfied.
    pub variable_bounds_satisfaction: Option<Vec<bool>>,
    /// Whether all constraints are satisfied.
    pub feasible: Option<bool>,
}

impl OwnedResult {
    pub fn new(
        sample: OwnedSample,
        obj_value: Bias,
        constraint_satisfaction: Vec<bool>,
        variable_bounds_satisfaction: Vec<bool>,
        feasible: bool,
    ) -> Self {
        Self {
            sample,
            obj_value: Some(obj_value),
            constraint_satisfaction: Some(constraint_satisfaction),
            variable_bounds_satisfaction: Some(variable_bounds_satisfaction),
            feasible: Some(feasible),
        }
    }

    pub fn get_sample(&self) -> Sample {
        Sample(Right(self.sample.clone()))
    }

    pub fn iter(&self) -> SampleIterator {
        SampleIterator::from_sample_vec(self.sample.clone())
    }
}

impl Display for ResultView {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = SolutionWriter::new()
            .write_sample(Sample(Left(self.clone())))
            .to_string();
        f.write_str(&s)
    }
}

impl Display for OwnedResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = SolutionWriter::new()
            .write_sample(Sample(Right(self.sample.clone())))
            .to_string();
        f.write_str(&s)
    }
}
