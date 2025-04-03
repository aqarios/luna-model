use crate::core::expression::BiasConstraints;
use crate::core::solution::base::AssignmentBaseTypes;
use crate::core::solution::sol::VarAssignment;
use crate::core::{IndexByValue, Solution};
use std::ops::Index;
use std::rc::Rc;

/// A view into a certain sample of a solution and its corresponding metadata.
#[derive(Debug, Clone)]
pub struct ResultView<Bias, AssignmentTypes>
where
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes,
{
    /// The solution this result view corresponds to
    sol: Rc<Solution<Bias, AssignmentTypes>>,
    /// Index of the row of the sample within the solution
    row_idx: usize,
}

/// Iterates over the single results of a solution
#[derive(Debug, Clone)]
pub struct ResultIterator<Bias, AssignmentTypes>
where
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes,
{
    /// The solution this result view corresponds to
    sol: Rc<Solution<Bias, AssignmentTypes>>,
    /// Index of the next row of the sample within the solution
    next_row: usize,
}

/// Iterates over the single variable assignments of a result
#[derive(Debug, Clone)]
pub struct SampleIterator<Bias, AssignmentTypes>
where
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes,
{
    /// The solution this result view corresponds to
    sol: Rc<Solution<Bias, AssignmentTypes>>,
    /// Index of the row of the sample within the solution
    row_idx: usize,
    /// Index of the next row of the sample within the solution
    next_col: usize,
}

impl<Bias, AssignmentTypes> ResultView<Bias, AssignmentTypes>
where
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes,
{
    pub fn new(sol: Rc<Solution<Bias, AssignmentTypes>>, row_idx: usize) -> Self {
        Self { sol, row_idx }
    }

    pub fn iter(&self) -> SampleIterator<Bias, AssignmentTypes> {
        SampleIterator::new(Rc::clone(&self.sol), self.row_idx)
    }

    pub fn obj_value(&self) -> Option<Bias> {
        self.sol
            .obj_values
            .get(self.row_idx)
            .map(|&b| b)
            .or_else(|| self.sol.raw_energies.get(self.row_idx).map(|&x| x))
    }

    pub fn constraint_satisfaction(&self) -> Option<&Vec<bool>> {
        self.sol.constraints.get(self.row_idx)
    }

    pub fn feasible(&self) -> Option<bool> {
        self.sol.feasible.get(self.row_idx).map(|&b| b)
    }

    pub fn get_assignment(&self, col_idx: usize) -> Option<VarAssignment<AssignmentTypes>> {
        self.sol.get_assignment(self.row_idx, col_idx)
    }
}

impl<Bias, AssignmentTypes> IndexByValue<usize> for ResultView<Bias, AssignmentTypes>
where
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes,
{
    type Output = VarAssignment<AssignmentTypes>;

    fn index_by_value(&self, index: usize) -> Self::Output {
        self.sol.get_assignment(self.row_idx, index).unwrap()
    }
}

impl<Bias, AssignmentTypes> ResultIterator<Bias, AssignmentTypes>
where
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes,
{
    pub fn new(sol: Rc<Solution<Bias, AssignmentTypes>>) -> Self {
        Self { sol, next_row: 0 }
    }
}

impl<Bias, AssignmentTypes> SampleIterator<Bias, AssignmentTypes>
where
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes,
{
    pub fn new(sol: Rc<Solution<Bias, AssignmentTypes>>, row_idx: usize) -> Self {
        Self {
            sol,
            row_idx,
            next_col: 0,
        }
    }
}

impl<Bias, AssignmentTypes> Iterator for ResultIterator<Bias, AssignmentTypes>
where
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes,
{
    type Item = ResultView<Bias, AssignmentTypes>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_row >= self.sol.len() {
            None
        } else {
            let res_view = Some(ResultView::new(Rc::clone(&self.sol), self.next_row));
            self.next_row += 1;
            res_view
        }
    }
}

impl<Bias, AssignmentTypes> Iterator for SampleIterator<Bias, AssignmentTypes>
where
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes,
{
    type Item = VarAssignment<AssignmentTypes>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.sol.get_assignment(self.row_idx, self.next_col);
        if let Some(_) = out {
            self.next_col += 1;
        }
        out
    }
}

pub struct OwnedResult<Bias, Assignment>
where
    Bias: BiasConstraints,
    Assignment: AssignmentBaseTypes,
{
    /// The vector of variable assignments.
    pub sample: Vec<VarAssignment<Assignment>>,
    /// The objective value computed from an AqModel. If not present, a raw value from the solver
    /// may be used. None, if none of these are present.
    pub obj_value: Option<Bias>,
    /// Boolean flag for each single constraint whether it's satisfied.
    pub constraint_satisfaction: Option<Vec<bool>>,
    /// Whether all constraints are satisfied.
    pub feasible: Option<bool>,
}
