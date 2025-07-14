use crate::core::environment::SharedEnvironment;
use crate::core::{SharedSolution, Solution, Timing};
use crate::errors::SolutionCreationErr;
use hashbrown::HashMap;
use num::NumCast;

pub struct DwaveTranslator {}

impl DwaveTranslator {
    pub fn from_dimod_sample_set<S, N, E>(
        samples: &[S],
        variables_order: &[String],
        counts: &[N],
        energy: &[E],
        shape: &[usize],
        timing: Option<Timing>,
        env: SharedEnvironment,
    ) -> Result<SharedSolution, SolutionCreationErr>
    where
        S: Copy + NumCast + Default,
        N: Copy + NumCast,
        E: Copy + NumCast,
    {
        let mut sol = Solution::default();
        sol.create_columns(&env, shape[0]);
        sol.timing = timing;
        sol.variable_names = env.variable_names();
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
        Ok(SharedSolution::from(sol))
    }
}
