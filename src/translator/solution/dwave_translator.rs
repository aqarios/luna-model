use crate::core::environment::SharedEnvironment;
use crate::core::expression::IndexConstraints;
use crate::core::solution::sol::SampleCol;
use crate::core::{RcSolution, Solution, Timing, Vtype};
use crate::errors::SolutionCreationErr;
use hashbrown::HashMap;
use num::NumCast;
use std::rc::Rc;

pub struct DwaveTranslator {}

impl DwaveTranslator {
    pub fn from_dimod_sample_set<S, N, E, Idx>(
        samples: &[S],
        variables_order: &[String],
        counts: &[N],
        energy: &[E],
        shape: &[usize],
        timing: Option<Timing>,
        env: SharedEnvironment
    ) -> Result<RcSolution, SolutionCreationErr>
    where
        S: Copy + NumCast + Default,
        N: Copy + NumCast,
        E: Copy + NumCast,
        Idx: IndexConstraints,
    {
        let mut sol = Solution::default();
        for v in env.borrow().iter() {
            match v.vtype {
                Vtype::Binary => sol.add_column(SampleCol::Binary(Vec::with_capacity(shape[0]))),
                Vtype::Spin => sol.add_column(SampleCol::Spin(Vec::with_capacity(shape[0]))),
                Vtype::Integer => sol.add_column(SampleCol::Integer(Vec::with_capacity(shape[0]))),
                Vtype::Real => sol.add_column(SampleCol::Real(Vec::with_capacity(shape[0]))),
            }
        }
        sol.timing = timing;
        sol.variable_names = env.borrow().iter().map(|v| v.name.clone()).collect();
        let map: HashMap<String, usize> = sol
            .variable_names
            .iter()
            .enumerate()
            .map(|(i, v)| (v.clone(), i))
            .collect();
        for i in 0..shape[0] {
            let start_idx = i * shape[1];
            let sample_unordered = samples[start_idx..start_idx + shape[1]].to_vec();
            let mut sample: Vec<S> = vec![S::default(); sample_unordered.len()];
            for (var, elem) in variables_order.iter().zip(sample_unordered) {
                sample[map[var]] = elem;
            }

            sol.extend(
                &sample,
                <usize as NumCast>::from(counts[i]).unwrap(),
                Some(energy[i]),
            )?;
        }
        Ok(RcSolution(Rc::new(sol)))
    }
}
