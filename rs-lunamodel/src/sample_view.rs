//! Owning snapshot view over one solution sample row.

use std::collections::HashMap;

use lunamodel_capi::LmSampleView;
use rs_lunamodel_derive::RawHandle;

use crate::{Assignment, Error, Result, Solution, Status, TryIndex, VarRef};

#[derive(Debug)]
pub enum SampleViewIdx {
    Num(usize),
    Var(VarRef),
    Str(String),
}

#[derive(RawHandle)]
pub struct SampleView {
    handle: *mut LmSampleView,
}

impl Drop for SampleView {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            free!(self, sample_view_free)
        }
    }
}

impl SampleView {
    pub fn new(solution: &Solution, index: usize) -> Result<Option<Self>> {
        solution.sample(index)
    }

    pub fn index(&self) -> Result<usize> {
        out!(sample_view_index(@self))
    }

    pub fn variable_names(&self) -> Result<Vec<String>> {
        out!(sample_view_variable_names(@self))
    }

    pub fn to_vec(&self) -> Result<Vec<Assignment>> {
        (0..self.variable_names()?.len())
            .map(|idx| out!(sample_view_assignment_idx(@self, idx)))
            .collect()
    }

    pub fn get(&self, var: &str) -> Result<Assignment> {
        out!(sample_view_assignment(@self, !var))
    }

    pub fn try_get(&self, var: SampleViewIdx) -> Result<Assignment> {
        match var {
            SampleViewIdx::Num(idx) => out!(sample_view_assignment_idx(@self, idx)),
            SampleViewIdx::Var(var) => self.get(&var.name()?),
            SampleViewIdx::Str(var) => self.get(&var),
        }
    }

    pub fn iter(&self) -> Result<std::vec::IntoIter<f64>> {
        Ok(self
            .to_vec()?
            .into_iter()
            .map(Assignment::value)
            .collect::<Vec<_>>()
            .into_iter())
    }

    pub fn iter_named(&self) -> Result<std::vec::IntoIter<(String, f64)>> {
        let names = self.variable_names()?;
        let values = self.to_vec()?;
        Ok(names
            .into_iter()
            .zip(values.into_iter().map(Assignment::value))
            .collect::<Vec<_>>()
            .into_iter())
    }

    pub fn as_map(&self) -> Result<HashMap<String, f64>> {
        Ok(self.iter_named()?.collect())
    }
}

impl TryFrom<(&Solution, usize)> for SampleView {
    type Error = Error;
    fn try_from((solution, index): (&Solution, usize)) -> Result<Self> {
        Self::new(solution, index)?.ok_or_else(|| Error {
            status: Status::Absent,
            message: format!("solution has no sample row at index {index}"),
            kind_tag: None,
        })
    }
}

impl TryIndex<&str> for SampleView {
    type Output = f64;

    fn try_index(&self, index: &str) -> Result<Self::Output> {
        Ok(self.get(index)?.value())
    }
}

impl std::fmt::Display for SampleView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.to_vec() {
            Ok(values) => write!(
                f,
                "[{}]",
                values
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Err(err) => write!(f, "<SampleView error: {err}>"),
        }
    }
}
