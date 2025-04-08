use crate::core::expression::BiasConstraints;
use crate::core::solution::base::AssignmentBaseTypes;
use crate::core::solution::sol::VarAssignment;
use crate::core::{IndexByValue, RcSolution};
use derive_more::{Deref, DerefMut};
use either::{Either, Left, Right};
use std::rc::Rc;

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

#[derive(Debug, Clone, Deref, DerefMut)]
pub struct Samples<Bias, AssignmentTypes>(pub RcSolution<Bias, AssignmentTypes>)
where
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes;

#[derive(Debug, Clone, Deref, DerefMut)]
pub struct Sample<Bias, AssignmentTypes>(
    pub Either<ResultView<Bias, AssignmentTypes>, Rc<Vec<VarAssignment<AssignmentTypes>>>>,
)
where
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes;

/// Iterates over the single results of a solution
#[derive(Debug, Clone)]
pub struct ResultIterator<Bias, AssignmentTypes>
where
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes,
{
    /// The solution this result view corresponds to
    sol: RcSolution<Bias, AssignmentTypes>,
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
    res: Either<ResultView<Bias, AssignmentTypes>, Rc<Vec<VarAssignment<AssignmentTypes>>>>,
    /// Index of the next row of the sample within the solution
    next_col_idx: usize,
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

    pub fn get_assignment(&self, col_idx: usize) -> Option<VarAssignment<AssignmentTypes>> {
        self.sol.get_assignment(self.row_idx, col_idx)
    }

    pub fn get_sample(&self) -> Sample<Bias, AssignmentTypes> {
        // Cloning is fine here as only usize and Rc are cloned.
        Sample(Left(self.clone()))
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
    pub fn new(sol: RcSolution<Bias, AssignmentTypes>) -> Self {
        Self { sol, next_row: 0 }
    }
}

impl<Bias, AssignmentTypes> SampleIterator<Bias, AssignmentTypes>
where
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes,
{
    pub fn from_res_view(res: &ResultView<Bias, AssignmentTypes>) -> Self {
        Self {
            res: Left(res.clone()),
            next_col_idx: 0,
        }
    }
    pub fn from_sample_vec(res: Rc<Vec<VarAssignment<AssignmentTypes>>>) -> Self {
        Self {
            res: Right(res),
            next_col_idx: 0,
        }
    }
}

impl<Bias, AssignmentTypes> Samples<Bias, AssignmentTypes>
where
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes,
{
    pub fn get_sample(&self, row_idx: usize) -> Option<Sample<Bias, AssignmentTypes>> {
        self.get_result_view(row_idx).map(|x| Sample(Left(x)))
    }

    pub fn get_assignment(
        &self,
        row_idx: usize,
        col_idx: usize,
    ) -> Option<VarAssignment<AssignmentTypes>> {
        self.0.get_assignment(row_idx, col_idx)
    }
}

impl<Bias, AssignmentTypes> Sample<Bias, AssignmentTypes>
where
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes,
{
    pub fn get_assignment(&self, col_idx: usize) -> Option<VarAssignment<AssignmentTypes>> {
        match &self.0 {
            Left(r) => r.get_assignment(col_idx),
            Right(r) => r.get(col_idx).map(|&x| x),
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
            let res_view = Some(ResultView::new(RcSolution::clone(&self.sol), self.next_row));
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
        let out = match &self.res {
            Left(r) => r.get_sample().get_assignment(self.next_col_idx),
            Right(r) => r.get(self.next_col_idx).map(|&x| x),
        };
        if let Some(_) = out {
            self.next_col_idx += 1;
        }
        out
    }
}

#[derive(Debug, Clone)]
pub struct OwnedResult<Bias, AssignmentTypes>
where
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes,
{
    /// The vector of variable assignments.
    pub sample: Rc<Vec<VarAssignment<AssignmentTypes>>>,
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
        sample: Rc<Vec<VarAssignment<AssignmentTypes>>>,
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
        Sample(Right(Rc::clone(&self.sample)))
    }

    pub fn iter(&self) -> SampleIterator<Bias, AssignmentTypes> {
        SampleIterator::from_sample_vec(Rc::clone(&self.sample))
    }
}
