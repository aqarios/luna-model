use std::{fmt::Display, ops::Index};

use itertools::Itertools;
use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::Bias;

use crate::{
    prelude::VarRef,
    solution::{Assignment, Solution},
    traits::TryIndex,
};

#[derive(Debug)]
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

    pub fn to_vec(&self) -> Vec<Assignment> {
        self.sol
            .variable_names()
            .iter()
            .map(|v| self.get(v))
            .collect()
    }

    pub fn get(&self, var: &str) -> Assignment {
        self.sol.assignment(self.idx, var)
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

impl<'s> TryIndex<&str> for SampleView<'s> {
    type Err = LunaModelError;
    type Output = Bias;

    fn try_index(&self, var: &str) -> Result<&Self::Output, Self::Err> {
        if self.sol.samples.contains_key(var) {
            Ok(&self.sol[(self.idx, var)])
        } else {
            Err(LunaModelError::VariableNotExisting(var.into()))
        }
    }
}

impl<'a> Display for SampleView<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}]",
            self.sol
                .variable_names()
                .iter()
                .map(|v| self.get(v).to_string())
                .join(", ")
        )
    }
}
