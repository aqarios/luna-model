use crate::core::expression::BiasConstraints;
use crate::core::solution::sample::OwnedSample;
use crate::core::solution::AssignmentBaseTypes;
use crate::core::{ResultView, VarAssignment};
use either::{Either, Left, Right};

/// Iterates over the single variable assignments of a solution row
#[derive(Debug, Clone)]
pub struct SampleIterator<Bias, AssignmentTypes>
where
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes,
{
    sample: Either<ResultView<Bias, AssignmentTypes>, OwnedSample<AssignmentTypes>>, // Rc<Vec<VarAssignment<AssignmentTypes>>>>,
    /// Index of the next row of the sample within the solution
    next_col_idx: usize,
}

impl<Bias, AssignmentTypes> SampleIterator<Bias, AssignmentTypes>
where
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes,
{
    pub fn new(
        sample: Either<ResultView<Bias, AssignmentTypes>, OwnedSample<AssignmentTypes>>, // Rc<Vec<VarAssignment<AssignmentTypes>>>>,
    ) -> SampleIterator<Bias, AssignmentTypes> {
        Self {
            sample,
            next_col_idx: 0,
        }
    }

    pub fn from_res_view(res: &ResultView<Bias, AssignmentTypes>) -> Self {
        Self {
            sample: Left(res.clone()),
            next_col_idx: 0,
        }
    }
    // pub fn from_sample_vec(res: Rc<Vec<VarAssignment<AssignmentTypes>>>) -> Self {
    pub fn from_sample_vec(res: OwnedSample<AssignmentTypes>) -> Self {
        Self {
            sample: Right(res),
            next_col_idx: 0,
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
        let out = match &self.sample {
            Left(r) => r.get_sample().get_assignment(self.next_col_idx),
            Right(r) => r.actual.get(self.next_col_idx).map(|&x| x),
        };
        if let Some(_) = out {
            self.next_col_idx += 1;
        }
        out
    }
}
