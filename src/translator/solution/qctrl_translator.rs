use num::NumCast;
use crate::{
    core::{SharedEnvironment, Solution, Timing},
    errors::SolutionCreationErr, types::Bias,
};

pub struct QctrlTranslator {}

impl QctrlTranslator {
    pub fn from_qctrl<S, E>(
        samples: Vec<Vec<S>>,
        counts: Vec<usize>,
        energies: Vec<E>,
        timing: Option<Timing>,
        env: SharedEnvironment,
    ) -> Result<Solution, SolutionCreationErr>
    where
        S: Copy + NumCast + Default,
        E: Copy + NumCast,
    {
        let mut sol = Solution::default();
        sol.create_columns(&env, samples.len());
        sol.timing = timing;
        sol.variable_names = env.variable_names();

        for ((sample, count), energy) in samples.iter().zip(counts).zip(energies) {
            sol.extend(&sample, count, <Bias as NumCast>::from(energy).unwrap())?;
        }
        Ok(sol)
    }
}

// Comment DB: Unneccesarty use Solution.from_counts
