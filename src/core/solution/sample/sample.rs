use super::{SampleIterator, VarAssignment};
use crate::{
    core::{
        solution::{result::ResultView, sol::Solution},
        ValueByIndex,
    },
    types::VarIndex,
};
use derive_more::{Deref, DerefMut};

pub enum Sample<'a> {
    View(SampleView<'a>),
    Owned(SampleOwned),
}

impl<'a> Sample<'a> {
    pub fn variable_names(&self) -> &[String] {
        match self {
            Self::View(view) => &view.sol.variable_names,
            Self::Owned(owned) => &owned.variable_names,
        }
    }

    /// The length of the sample is equal to the number of
    /// variable assignments or the number of variables in
    /// the solution, so also equal to the number of cols.
    pub fn len(&self) -> usize {
        match self {
            Self::View(view) => view.sol.samples.len(),
            Self::Owned(owned) => owned.actual.len(),
        }
    }

    pub fn iter(&'a self) -> SampleIterator<'a> {
        SampleIterator::new(self)
    }
}

impl<'a> ValueByIndex<VarIndex> for Sample<'a> {
    type Output = VarAssignment;

    fn value_by_index(&self, index: VarIndex) -> Self::Output {
        match self {
            Self::View(view) => view.value_by_index(index),
            Self::Owned(owned) => owned.value_by_index(index),
        }
    }
}

// VIEW
#[derive(Debug, Clone, Deref, DerefMut)]
pub struct SampleView<'a> {
    pub res: ResultView<'a>,
}

impl<'a> SampleView<'a> {
    pub fn new(sol: &'a Solution, row: usize) -> Self {
        Self {
            res: ResultView::new(sol, row),
        }
    }
}

impl<'a> ValueByIndex<VarIndex> for SampleView<'a> {
    type Output = VarAssignment;

    fn value_by_index(&self, index: VarIndex) -> Self::Output {
        self.res.value_by_index(index)
    }
}

// OWNED
#[derive(Debug, Clone)]
pub struct SampleOwned {
    pub variable_names: Vec<String>,
    pub actual: Vec<VarAssignment>,
}

impl SampleOwned {
    pub fn new(variable_names: Vec<String>, actual: Vec<VarAssignment>) -> Self {
        Self {
            variable_names,
            actual,
        }
    }
}

impl<'a> ValueByIndex<VarIndex> for SampleOwned {
    type Output = VarAssignment;

    fn value_by_index(&self, index: VarIndex) -> Self::Output {
        let idx: usize = index.into();
        self.actual[idx]
    }
}
