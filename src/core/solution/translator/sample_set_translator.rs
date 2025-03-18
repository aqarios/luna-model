use crate::core::solution::Solution;

pub struct SampleSetTranslator {}

impl SampleSetTranslator {
    pub fn from_dimod_sample_set(
        samples: &[i64],
        num_occurrences: &[i64],
        shape: &[usize],
    ) -> Solution<f64, f64>
where {
        let mut sol = Solution::default();
        for i in 0..shape[0] {
            let start_idx = i * shape[1];
            let sample = samples[start_idx..start_idx + shape[1]]
                .iter()
                .map(|&x| x as f64)
                .collect();
            sol.extend(sample, num_occurrences[i] as usize);
        }
        sol
    }
}
