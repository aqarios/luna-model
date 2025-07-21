use std::{
    fmt::{Debug, Display},
};

use num::NumCast;

use crate::{
    core::{SharedEnvironment, Solution, Timing, VarRef},
    errors::SolutionCreationErr,
    types::Bias, utils::Share,
};

pub struct IbmTranslator {}

impl IbmTranslator {
    pub fn from_ibm<S, E>(
        samples: &Vec<Vec<S>>,
        orderings: &Vec<Share<VarRef>>,
        energies: &Vec<E>,
        counts: Vec<usize>,
        timing: Option<Timing>,
        env: SharedEnvironment,
    ) -> Result<Solution, SolutionCreationErr>
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
            sol.extend(&s, occ, Some(<Bias as NumCast>::from(*energy).unwrap()))?;
        }
        Ok(sol)
    }
}

// Comment DB: unneccesary use Solution.from_counts
// Alternatively: direct translation of the BitArray sample object
