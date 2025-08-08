use crate::core::environment::SharedEnvironment;
use crate::core::{Solution, Timing};
use crate::errors::SolutionCreationErr;
use crate::types::Bias;
use num::NumCast;

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
    ) -> Result<Solution, SolutionCreationErr>
    where
        S: Copy + NumCast,
        N: Copy + NumCast,
        E: Copy + NumCast,
    {
        let mut sol = Solution::default();
        sol.create_columns(&env, shape[0]);
        sol.timing = timing;
        sol.variable_names = env.variable_names();
        for i in 0..shape[0] {
            let start_idx = i * shape[1];
            let sample = samples[start_idx..start_idx + shape[1]].to_vec();
            sol.extend(
                &sample,
                <usize as NumCast>::from(counts[i]).unwrap(),
                <Bias as NumCast>::from(energies[indices[i]]).unwrap(),
            )?;
        }
        Ok(sol)
    }
}
