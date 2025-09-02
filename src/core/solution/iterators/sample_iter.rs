use crate::core::solution::sample::OwnedSample;
use crate::core::{ResultView, VarAssignment};
use either::{Either, Left, Right};

/// Iterates over the single variable assignments of a solution row
#[derive(Debug, Clone)]
pub struct SampleIterator {
    sample: Either<ResultView, OwnedSample>,
    /// Index of the next row of the sample within the solution
    next_col_idx: usize,
}

impl SampleIterator {
    pub fn new(sample: Either<ResultView, OwnedSample>) -> SampleIterator {
        Self {
            sample,
            next_col_idx: 0,
        }
    }

    pub fn from_res_view(res: &ResultView) -> Self {
        Self {
            sample: Left(res.clone()),
            next_col_idx: 0,
        }
    }

    pub fn from_sample_vec(res: OwnedSample) -> Self {
        Self {
            sample: Right(res),
            next_col_idx: 0,
        }
    }
}

impl Iterator for SampleIterator {
    type Item = VarAssignment;

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
