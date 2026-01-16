use std::ops::Index;

use lunamodel_error::LunaModelResult;
use lunamodel_types::Bias;

use crate::{
    prelude::VarRef,
    solution::{Assignment, Solution},
};

pub enum SampleViewIdx {
    Num(usize),
    Var(VarRef),
    Str(String),
}

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

    pub fn try_get(&self, var: SampleViewIdx) -> LunaModelResult<Assignment> {
        match var {
            SampleViewIdx::Num(v) => self.sol.try_assignment_idx(self.idx, v),
            SampleViewIdx::Var(v) => self.sol.try_assignment(self.idx, &v.name()?),
            SampleViewIdx::Str(v) => self.sol.try_assignment(self.idx, &v),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = Bias> {
        self.sol
            .samples
            .iter()
            .map(|(_, samples)| samples[self.idx])
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
