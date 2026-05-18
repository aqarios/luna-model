//! View type for one raw sample row inside a solution.

use std::{collections::HashMap, fmt::Display, ops::Index};

use itertools::Itertools;
use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::Bias;

use crate::{
    prelude::VarRef,
    solution::{Assignment, Solution},
    traits::TryIndex,
};

/// Selector type for addressing a value inside a [`SampleView`].
#[derive(Debug)]
pub enum SampleViewIdx {
    /// Select by column position.
    Num(usize),
    /// Select by a variable reference.
    Var(VarRef),
    /// Select by variable name.
    Str(String),
}

/// Borrowed view over one solution row.
#[derive(Debug)]
pub struct SampleView<'s> {
    /// Borrowed source solution.
    pub sol: &'s Solution,
    /// Row index inside the source solution.
    pub idx: usize,
}

impl<'s> SampleView<'s> {
    /// Creates a new sample view over `sol[idx]`.
    pub fn new(sol: &'s Solution, idx: usize) -> Self {
        Self { sol, idx }
    }

    /// Materializes the row as typed assignments in column order.
    pub fn to_vec(&self) -> Vec<Assignment> {
        self.sol
            .variable_names()
            .iter()
            .map(|v| self.get(v))
            .collect()
    }

    /// Returns the typed assignment for a variable name.
    pub fn get(&self, var: &str) -> Assignment {
        self.sol.assignment(self.idx, var)
    }

    /// Fallible generalized row lookup by selector.
    pub fn try_get(&self, var: SampleViewIdx) -> LunaModelResult<Assignment> {
        match var {
            SampleViewIdx::Num(v) => self.sol.try_assignment_idx(self.idx, v),
            SampleViewIdx::Var(v) => self.sol.try_assignment(self.idx, &v.name()?),
            SampleViewIdx::Str(v) => self.sol.try_assignment(self.idx, &v),
        }
    }

    /// Iterates over the raw numeric row values in column order.
    pub fn iter(&self) -> impl Iterator<Item = Bias> {
        self.sol
            .samples
            .iter()
            .map(|(_, samples)| samples[self.idx])
    }

    /// Iterates over the raw numeric row values in column order with var name.
    pub fn iter_named(&self) -> impl Iterator<Item = (&String, Bias)> {
        self.sol
            .samples
            .iter()
            .map(|(name, samples)| (name, samples[self.idx]))
    }

    pub fn as_map(&self) -> HashMap<String, Bias> {
        self.iter_named()
            .map(|(name, val)| (name.clone(), val))
            .collect()
    }
}

impl<'s> From<(&'s Solution, usize)> for SampleView<'s> {
    /// Creates a sample view from a `(solution, row)` pair.
    fn from(value: (&'s Solution, usize)) -> Self {
        let (sol, idx) = value;
        Self::new(sol, idx)
    }
}

impl<'s> Index<&str> for SampleView<'s> {
    type Output = Bias;

    /// Indexes the row by variable name.
    fn index(&self, var: &str) -> &Self::Output {
        &self.sol[(self.idx, var)]
    }
}

impl<'s> TryIndex<&str> for SampleView<'s> {
    type Err = LunaModelError;
    type Output = Bias;

    /// Fallible variable lookup by name.
    fn try_index(&self, var: &str) -> Result<&Self::Output, Self::Err> {
        if self.sol.samples.contains_key(var) {
            Ok(&self.sol[(self.idx, var)])
        } else {
            Err(LunaModelError::VariableNotExisting(var.into()))
        }
    }
}

impl<'a> Display for SampleView<'a> {
    /// Formats the row as a flat assignment list in column order.
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
