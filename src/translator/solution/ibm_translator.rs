use std::{
    fmt::{Debug, Display},
    rc::Rc,
};

use num::NumCast;

use crate::{
    core::{environment::SharedEnvironment, RcSolution, Solution, Timing, VarRef},
    errors::SolutionCreationErr,
};

pub struct IbmTranslator {}

impl IbmTranslator {
    pub fn from_ibm<S, E>(
        samples: &Vec<Vec<S>>,
        orderings: &Vec<Rc<VarRef>>,
        energies: &Vec<E>,
        counts: Vec<usize>,
        timing: Option<Timing>,
        env: SharedEnvironment,
    ) -> Result<RcSolution, SolutionCreationErr>
    where
        S: Copy + NumCast + Default + Display + Debug,
        E: Copy + NumCast + Debug,
    {
        let mut sol = Solution::default();
        sol.create_columns(&env, 1);
        sol.timing = timing;
        sol.variable_names = env.variable_names();
        // used to determine the order of each assignment in the sample.
        let index_list: Vec<usize> = orderings.iter().map(|e| e.id.into()).collect();
        // For each sample:
        // Map the sample to the correct order.
        for ((sample, energy), occ) in samples.iter().zip(energies).zip(counts) {
            let mut s: Vec<S> = vec![S::default(); sample.len()];
            for (&idx, val) in index_list.iter().zip(sample) {
                s[idx] = *val;
            }
            sol.extend(&s, occ, Some(*energy))?;
        }
        Ok(RcSolution(Rc::new(sol)))
    }
}

// Comment DB: unneccesary use Solution.from_counts
// Alternatively: direct translation of the BitArray sample object
