use crate::core::environment::SharedEnvironment;
use crate::core::solution::sol::SampleCol;
use crate::core::{RcSolution, Solution, Timing, Vtype};
use crate::errors::SolutionCreationErr;
use num::NumCast;
use std::rc::Rc;

pub struct NpArrayTranslator {}

impl NpArrayTranslator {
    pub fn from_numpy_arrays<S, N, E>(
        samples: &[S],
        counts: &[N],
        indices: &[usize],
        energies: &[E],
        shape: &[usize],
        timing: Option<Timing>,
        env: SharedEnvironment,
    ) -> Result<RcSolution, SolutionCreationErr>
    where
        S: Copy + NumCast,
        N: Copy + NumCast,
        E: Copy + NumCast,
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
        sol.variable_names = env.borrow().iter().map(|v| v.name.clone()).collect();
        for i in 0..shape[0] {
            let start_idx = i * shape[1];
            let sample = samples[start_idx..start_idx + shape[1]].to_vec();
            sol.extend(
                &sample,
                <usize as NumCast>::from(counts[i]).unwrap(),
                Some(energies[indices[i]]),
            )?;
        }
        Ok(RcSolution(Rc::new(sol)))
    }
}
