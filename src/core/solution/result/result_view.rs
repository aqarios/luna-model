use crate::{
    core::{solution::sol::Solution, ValueByIndex, VarAssignment},
    types::VarIndex,
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
