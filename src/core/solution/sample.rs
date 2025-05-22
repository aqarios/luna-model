use crate::core::expression::{BiasConstraints, IndexConstraints};
use crate::core::solution::AssignmentBaseTypes;
use crate::core::writer::SolutionWriter;
use crate::core::{
    ValueByIndex, RcSolution, ResultView, SampleIterator, SamplesIterator, VarAssignment,
};
use derive_more::{Deref, DerefMut};
use either::{Either, Left, Right};
use std::fmt::{Display, Formatter};
use std::rc::Rc;

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

    pub fn iter(&self) -> SamplesIterator<Bias, AssignmentTypes> {
        SamplesIterator::new(RcSolution::clone(&self.0))
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

    pub fn iter(&self) -> SampleIterator<Bias, AssignmentTypes> {
        SampleIterator::new(self.0.clone())
    }
}

impl<Bias, AssignmentTypes, Index> ValueByIndex<Index> for Sample<Bias, AssignmentTypes>
where
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes,
    Index: IndexConstraints,
{
    type Output = VarAssignment<AssignmentTypes>;

    fn value_by_index(&self, index: Index) -> Self::Output {
        match &self.0 {
            Left(r) => r.value_by_index(index),
            Right(s) => s[index.into()],
        }
    }
}

impl<Bias, AssignmentTypes> IntoIterator for Sample<Bias, AssignmentTypes>
where
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes,
{
    type Item = VarAssignment<AssignmentTypes>;
    type IntoIter = SampleIterator<Bias, AssignmentTypes>;

    fn into_iter(self) -> Self::IntoIter {
        SampleIterator::new(self.0)
    }
}

impl<Bias, AssignmentTypes> Display for Samples<Bias, AssignmentTypes>
where
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = SolutionWriter::new()
            .write_samples(self.clone(), &self.counts)
            .to_string();
        f.write_str(&s)
    }
}

impl<Bias, AssignmentTypes> Display for Sample<Bias, AssignmentTypes>
where
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = SolutionWriter::new().write_sample(self.clone()).to_string();
        f.write_str(&s)
    }
}
