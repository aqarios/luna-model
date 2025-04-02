use crate::core::expression::{BiasConstraints, IndexConstraints};
use crate::core::solution::base::AssignmentBaseTypes;
use crate::core::solution::sol::VarAssignment;
use crate::core::Solution;
use std::rc::Rc;

/// A view into a certain sample of a solution and its corresponding metadata.
#[derive(Debug, Clone)]
pub struct ResultView<Idx, Bias, AssignmentTypes>
where
    Idx: IndexConstraints,
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes,
{
    /// The solution this result view corresponds to
    sol: Rc<Solution<Bias, AssignmentTypes>>,
    /// Index of the row of the sample within the solution
    row_idx: Idx,
}

/// Iterates over the single results of a solution
#[derive(Debug, Clone)]
pub struct ResultIterator<Idx, Bias, AssignmentTypes>
where
    Idx: IndexConstraints,
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes,
{
    /// The solution this result view corresponds to
    sol: Rc<Solution<Bias, AssignmentTypes>>,
    /// Index of the next row of the sample within the solution
    next_row: Idx,
}

/// Iterates over the single variable assignments of a result
#[derive(Debug, Clone)]
pub struct SampleIterator<Idx, Bias, AssignmentTypes>
where
    Idx: IndexConstraints,
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes,
{
    /// The solution this result view corresponds to
    sol: Rc<Solution<Bias, AssignmentTypes>>,
    /// Index of the row of the sample within the solution
    row_idx: Idx,
    /// Index of the next row of the sample within the solution
    next_col: Idx,
}

impl<Idx, Bias, AssignmentTypes> ResultView<Idx, Bias, AssignmentTypes>
where
    Idx: IndexConstraints,
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes,
{
    pub fn new(sol: Rc<Solution<Bias, AssignmentTypes>>, row_idx: Idx) -> Self {
        Self { sol, row_idx }
    }

    pub fn iter(&self) -> SampleIterator<Idx, Bias, AssignmentTypes> {
        SampleIterator::new(Rc::clone(&self.sol), self.row_idx)
    }

    pub fn obj_value(&self) -> Option<Bias> {
        self.sol
            .obj_values
            .get(self.row_idx.into())
            .map(|&b| b)
            .or_else(|| self.sol.raw_energies.get(self.row_idx.into()).map(|&x| x))
    }

    pub fn constraint_satisfaction(&self) -> Option<&Vec<bool>> {
        self.sol.constraints.get(self.row_idx.into())
    }

    pub fn feasible(&self) -> Option<bool> {
        self.sol.feasible.get(self.row_idx.into()).map(|&b| b)
    }
}

impl<Idx, Bias, AssignmentTypes> ResultIterator<Idx, Bias, AssignmentTypes>
where
    Idx: IndexConstraints,
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes,
{
    pub fn new(sol: Rc<Solution<Bias, AssignmentTypes>>) -> Self {
        Self {
            sol,
            next_row: Idx::default(),
        }
    }
}

impl<Idx, Bias, AssignmentTypes> SampleIterator<Idx, Bias, AssignmentTypes>
where
    Idx: IndexConstraints,
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes,
{
    pub fn new(sol: Rc<Solution<Bias, AssignmentTypes>>, row_idx: Idx) -> Self {
        Self {
            sol,
            row_idx,
            next_col: Idx::default(),
        }
    }
}

impl<Idx, Bias, AssignmentTypes> Iterator for ResultIterator<Idx, Bias, AssignmentTypes>
where
    Idx: IndexConstraints,
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes,
{
    type Item = ResultView<Idx, Bias, AssignmentTypes>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_row.into() >= self.sol.len() {
            None
        } else {
            let res_view = Some(ResultView::new(Rc::clone(&self.sol), self.next_row));
            self.next_row += Idx::one();
            res_view
        }
    }
}

impl<Idx, Bias, AssignmentTypes> Iterator for SampleIterator<Idx, Bias, AssignmentTypes>
where
    Idx: IndexConstraints,
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes,
{
    type Item = VarAssignment<AssignmentTypes>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.sol.get_assignment(self.row_idx, self.next_col);
        if let Some(_) = out {
            self.next_col += Idx::one();
        }
        out
    }
}

// pub struct OwnedResult<Assignment, Bias>
// where
//     Assignment: AssignmentBaseTypes,
//     Bias: BiasConstraints,
// {
//     /// The vector of variable assignments.
//     pub sample: Sample<Assignment>,
//     /// The objective value computed from an AqModel. If not present, a raw value from the solver
//     /// may be used. None, if none of these are present.
//     pub obj_value: Option<Bias>,
//     /// Boolean flag for each single constraint whether it's satisfied.
//     pub constraint_satisfaction: Option<Vec<bool>>,
//     /// Whether all constraints are satisfied.
//     pub feasible: Option<bool>,
// }
