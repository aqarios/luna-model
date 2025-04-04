use crate::core::expression::IndexConstraints;
use crate::core::solution::sol::SampleCol;
use crate::core::{ConcreteSolution, MutRcEnvironment, RcSolution, Solution, Timing, Vtype};
use crate::errors::SampleIncorrectLengthError;
use num::NumCast;
use std::fmt::Debug;
use std::rc::Rc;

pub struct SampleSetTranslator {}

impl SampleSetTranslator {
    pub fn from_dimod_sample_set<S, N, Idx>(
        samples: &[S],
        num_occurrences: &[N],
        shape: &[usize],
        timing: Option<Timing>,
        env: MutRcEnvironment<Idx>,
    ) -> Result<ConcreteSolution, SampleIncorrectLengthError>
    where
        S: Copy + NumCast + Debug,
        N: Copy + NumCast + Debug,
        Idx: IndexConstraints,
    {
        let mut sol = Solution::default();
        for v in env.borrow().variables.iter() {
            match v.vtype {
                Vtype::Binary => sol.add_column(SampleCol::Binary(Vec::with_capacity(shape[0]))),
                Vtype::Spin => sol.add_column(SampleCol::Spin(Vec::with_capacity(shape[0]))),
                Vtype::Integer => sol.add_column(SampleCol::Integer(Vec::with_capacity(shape[0]))),
                Vtype::Real => sol.add_column(SampleCol::Real(Vec::with_capacity(shape[0]))),
            }
        }
        sol.timing = timing;
        for i in 0..shape[0] {
            let start_idx = i * shape[1];
            let sample = samples[start_idx..start_idx + shape[1]]
                .iter()
                .map(|&x| x)
                .collect();
            sol.extend(
                sample,
                <usize as NumCast>::from(num_occurrences[i]).unwrap(),
            )?;
        }
        Ok(RcSolution(Rc::new(sol)))
    }
}

