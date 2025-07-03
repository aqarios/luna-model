use std::rc::Rc;

use num::NumCast;

use crate::{
    core::{environment::SharedEnvironment, RcSolution, Solution, Timing},
    errors::SolutionCreationErr,
};

pub struct QctrlTranslator {}

impl QctrlTranslator {
    pub fn from_qctrl<S, E>(
        samples: Vec<Vec<S>>,
        counts: Vec<usize>,
        energies: Vec<Option<E>>,
        timing: Option<Timing>,
        env: SharedEnvironment,
    ) -> Result<RcSolution, SolutionCreationErr>
    where
        S: Copy + NumCast + Default,
        E: Copy + NumCast,
    {
        let mut sol = Solution::default();
        sol.create_columns(&env, samples.len());
        sol.timing = timing;
        sol.variable_names = env.variable_names();

        for ((sample, count), energy) in samples.iter().zip(counts).zip(energies) {
            sol.extend(&sample, count, energy)?;
        }
        Ok(RcSolution(Rc::new(sol)))
    }
}

// Comment DB: Unneccesarty use Solution.from_counts
