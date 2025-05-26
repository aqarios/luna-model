use crate::core::expression::{BiasConstraints, IndexConstraints};
use crate::core::solution::base::AssignmentBaseTypes;
use crate::core::solution::sol::VarAssignment;
use crate::core::writer::SolutionWriter;
use crate::core::{RcSolution, Sample, SampleIterator, ValueByIndex};
use either::{Left, Right};
use std::fmt::{Display, Formatter};

use super::sample::OwnedSample;

/// A view into a certain sample of a solution and its corresponding metadata.
#[derive(Debug, Clone)]
pub struct ResultView<Bias, AssignmentTypes>
where
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes,
{
    /// The solution this result view corresponds to
    pub sol: RcSolution<Bias, AssignmentTypes>,
    /// Index of the row of the sample within the solution
    pub row_idx: usize,
}

impl<Bias, AssignmentTypes> ResultView<Bias, AssignmentTypes>
where
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes,
{
    pub fn new(sol: RcSolution<Bias, AssignmentTypes>, row_idx: usize) -> Self {
        Self { sol, row_idx }
    }

    pub fn iter(&self) -> SampleIterator<Bias, AssignmentTypes> {
        SampleIterator::from_res_view(&self)
    }

    pub fn obj_value(&self) -> Option<Bias> {
        self.sol.obj_values[self.row_idx]
    }

    pub fn raw_energy(&self) -> Option<Bias> {
        self.sol.raw_energies[self.row_idx]
    }

    pub fn constraint_satisfaction(&self) -> &Option<Vec<bool>> {
        &self.sol.constraints[self.row_idx]
    }

    pub fn feasible(&self) -> Option<bool> {
        self.sol.feasible[self.row_idx]
    }

    pub fn counts(&self) -> usize {
        self.sol.counts[self.row_idx]
    }

    pub fn get_assignment(&self, col_idx: usize) -> Option<VarAssignment<AssignmentTypes>> {
        self.sol.get_assignment(self.row_idx, col_idx)
    }

    pub fn get_sample(&self) -> Sample<Bias, AssignmentTypes> {
        // Cloning is fine here as only usize and Rc are cloned.
        Sample(Left(self.clone()))
    }
}

impl<Bias: BiasConstraints, AssignmentTypes: AssignmentBaseTypes + PartialEq> PartialEq
    for ResultView<Bias, AssignmentTypes>
{
    fn eq(&self, other: &Self) -> bool {
        self.row_idx == self.row_idx && self.sol == other.sol
    }
}

impl<Bias, AssignmentTypes, Index> ValueByIndex<Index> for ResultView<Bias, AssignmentTypes>
where
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes,
    Index: IndexConstraints,
{
    type Output = VarAssignment<AssignmentTypes>;

    fn value_by_index(&self, index: Index) -> Self::Output {
        self.sol.get_assignment(self.row_idx, index.into()).unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct OwnedResult<Bias, AssignmentTypes>
where
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes,
{
    /// The vector of variable assignments.
    pub sample: OwnedSample<AssignmentTypes>, // Rc<Vec<VarAssignment<AssignmentTypes>>>,
    /// The objective value computed from an AqModel. If not present, a raw value from the solver
    /// may be used. None, if none of these are present.
    pub obj_value: Option<Bias>,
    /// Boolean flag for each single constraint whether it's satisfied.
    pub constraint_satisfaction: Option<Vec<bool>>,
    /// Whether all constraints are satisfied.
    pub feasible: Option<bool>,
}

impl<Bias, AssignmentTypes> OwnedResult<Bias, AssignmentTypes>
where
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes,
{
    pub fn new(
        sample: OwnedSample<AssignmentTypes>, // Rc<Vec<VarAssignment<AssignmentTypes>>>,
        obj_value: Bias,
        constraint_satisfaction: Vec<bool>,
        feasible: bool,
    ) -> Self {
        Self {
            sample,
            obj_value: Some(obj_value),
            constraint_satisfaction: Some(constraint_satisfaction),
            feasible: Some(feasible),
        }
    }

    pub fn get_sample(&self) -> Sample<Bias, AssignmentTypes> {
        Sample(Right(self.sample.clone()))
    }

    pub fn iter(&self) -> SampleIterator<Bias, AssignmentTypes> {
        SampleIterator::from_sample_vec(self.sample.clone())
    }
}

impl<Bias, AssignmentTypes> Display for ResultView<Bias, AssignmentTypes>
where
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = SolutionWriter::new()
            .write_sample(Sample(Left(self.clone())))
            .to_string();
        f.write_str(&s)
    }
}

impl<Bias, AssignmentTypes> Display for OwnedResult<Bias, AssignmentTypes>
where
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = SolutionWriter::<Bias, AssignmentTypes>::new()
            .write_sample(Sample(Right(self.sample.clone())))
            .to_string();
        f.write_str(&s)
    }
}
