use std::ops::Index;

use lunamodel_types::Bias;

use crate::solution::Solution;

#[derive(Debug)]
pub struct SampleView<'s> {
    pub sol: &'s Solution,
    pub idx: usize,
}

impl<'s> SampleView<'s> {
    pub fn new(sol: &'s Solution, idx: usize) -> Self {
        Self { sol, idx }
    }

    pub fn to_vec(&self) -> Vec<f64> {
        self.sol.variable_names().iter().map(|v| self[v]).collect()
    }
}

impl<'s> From<(&'s Solution, usize)> for SampleView<'s> {
    fn from(value: (&'s Solution, usize)) -> Self {
        let (sol, idx) = value;
        Self::new(sol, idx)
    }
}

impl<'s> Index<&str> for SampleView<'s> {
    type Output = Bias;

    fn index(&self, var: &str) -> &Self::Output {
        &self.sol[(self.idx, var)]
    }
}
