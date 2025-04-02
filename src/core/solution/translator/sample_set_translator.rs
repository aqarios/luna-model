use crate::core::{ConcreteSolution, Solution, Timing};
use num::NumCast;

pub struct SampleSetTranslator {}

impl SampleSetTranslator {
    pub fn from_dimod_sample_set<S, N>(
        samples: &[S],
        num_occurrences: &[N],
        shape: &[usize],
        timing: Option<Timing>,
    ) -> Result<ConcreteSolution, ()>
    where
        S: Copy + NumCast,
        N: Copy + NumCast,
    {
        let mut sol = Solution::default();
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
        Ok(sol)
    }
}
