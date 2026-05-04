//! Read-only accessors and views over solutions.

use std::ops::Index;

use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::{Bias, Sense};

use crate::solution::Assignment;

use super::{Solution, result::ResultView, sample::SampleView};

const RTOL: f64 = f64::EPSILON;
const ATOL: f64 = f64::EPSILON;

fn is_close(a: f64, b: f64, rtol: f64, atol: f64) -> bool {
    let diff = (a - b).abs();
    if diff <= atol {
        return true;
    }

    let largest = a.abs().max(b.abs());
    diff <= largest * rtol
}

impl Solution {
    /// Returns the total number of sample occurrences represented by the solution.
    ///
    /// This is the sum of `counts`, not the number of unique sample rows. After
    /// aggregation, a single stored row may still represent multiple identical
    /// solver outputs.
    pub fn n_samples(&self) -> usize {
        self.counts.iter().sum()
    }

    /// Returns the number of stored sample rows.
    ///
    /// This corresponds to the length of each column in [`Solution::samples`].
    pub fn len(&self) -> usize {
        match self.samples.first() {
            Some((_, col)) => col.len(),
            _ => 0,
        }
    }

    /// Returns `true` if the solution stores no sample rows.
    pub fn is_empty(&self) -> bool {
        match self.samples.first() {
            Some((_, col)) => col.is_empty(),
            _ => false,
        }
    }

    /// Returns the number of variables represented by the solution.
    pub fn sample_len(&self) -> usize {
        self.samples.len()
    }

    /// Returns the value for `var` in the given sample row.
    ///
    /// This is a convenience accessor over the column-oriented storage.
    pub fn value(&self, sample: usize, var: &str) -> Option<&Bias> {
        match sample >= self.len() {
            true => None,
            false => Some(&self[(sample, var)]),
        }
    }

    /// Returns the typed assignment for `var` in the given sample row.
    pub fn assignment(&self, sample: usize, var: &str) -> Assignment {
        self.samples[var].as_assignment(sample)
    }

    /// Fallible version of [`Solution::assignment`].
    pub fn try_assignment(&self, sample: usize, var: &str) -> LunaModelResult<Assignment> {
        if sample >= self.len() {
            return Err(LunaModelError::IndexOutOfBounds(
                format!("{}, is {}", sample, self.len()).into(),
            ));
        }
        if !self.samples.contains_key(var) {
            return Err(LunaModelError::VariableNotExisting(var.into()));
        }
        Ok(self.samples[var].as_assignment(sample))
    }

    /// Returns a typed assignment by sample row and column position.
    ///
    /// This is mostly useful in lower-level iteration code that is already
    /// walking the column order rather than named lookup.
    pub fn try_assignment_idx(&self, sample: usize, var: usize) -> LunaModelResult<Assignment> {
        if sample >= self.len() {
            return Err(LunaModelError::IndexOutOfBounds(
                format!("{}, number of samples is {}", sample, self.len()).into(),
            ));
        }
        if var >= self.samples.len() {
            return Err(LunaModelError::IndexOutOfBounds(
                format!("{}, sample length is {}", var, self.sample_len()).into(),
            ));
        }
        Ok(self.samples[var].as_assignment(sample))
    }

    /// Returns a result-oriented view for a sample row.
    pub fn result(&self, index: usize) -> Option<ResultView<'_>> {
        match index >= self.len() {
            true => None,
            false => Some((self, index).into()),
        }
    }

    /// Iterates over all sample rows as result-oriented views.
    pub fn results<'s>(&'s self) -> impl Iterator<Item = ResultView<'s>> {
        (0..self.len()).map(move |i| (self, i).into())
    }

    /// Returns a sample-oriented view for a sample row.
    pub fn sample(&self, index: usize) -> Option<SampleView<'_>> {
        match index >= self.len() {
            true => None,
            false => Some((self, index).into()),
        }
    }

    /// Iterates over all sample rows as sample-oriented views.
    pub fn samples<'s>(&'s self) -> impl Iterator<Item = SampleView<'s>> {
        (0..self.len()).map(move |i| (self, i).into())
    }

    /// Returns the stored variable names in column order.
    pub fn variable_names(&self) -> Vec<String> {
        self.samples.keys().cloned().collect()
    }

    /// Returns all best feasible results within floating-point tolerance.
    ///
    /// The solution may contain multiple rows with effectively identical
    /// objective values. Rather than returning a single arbitrary winner, this
    /// method returns all rows that are close to the best feasible objective.
    pub fn best(&self) -> Option<Vec<ResultView<'_>>> {
        match (&self.feasible, &self.obj_values) {
            (Some(f), Some(ov)) => {
                let target_map = ov.iter().zip(f).filter(|(_, f)| **f).map(|(a, _)| a);
                let target = match self.sense {
                    Sense::Min => target_map.min_by(|&a, &b| a.total_cmp(b)),
                    Sense::Max => target_map.max_by(|&a, &b| a.total_cmp(b)),
                };
                target?;
                let target = *target.unwrap();
                let alltargets: Vec<usize> = ov
                    .iter()
                    .zip(f)
                    .enumerate()
                    .filter(|(_, (_, f))| **f)
                    .map(|(idx, (val, _))| (idx, val))
                    .filter(|(_, val)| is_close(**val, target, RTOL, ATOL))
                    .map(|(idx, _)| idx)
                    .collect();
                let views = alltargets
                    .iter()
                    .map(|idx| ResultView::new(self, *idx))
                    .collect();
                Some(views)
            }
            _ => None,
        }
    }

    /// Extracts a single sample row as a standalone one-row solution.
    pub fn extract(&self, row: usize) -> Solution {
        Solution {
            samples: self
                .samples
                .iter()
                .map(|(v, c)| (v.clone(), c.extract(row)))
                .collect(),
            counts: vec![self.counts[row]],
            raw_energies: self.raw_energies.as_ref().map(|e| vec![e[row]]),
            obj_values: self.obj_values.as_ref().map(|e| vec![e[row]]),
            feasible: self.feasible.as_ref().map(|e| vec![e[row]]),
            constraints: self
                .constraints
                .iter()
                .map(|(cname, cs)| (cname.clone(), vec![cs[row]]))
                .collect(),
            variable_bounds: self
                .variable_bounds
                .iter()
                .map(|(vname, cs)| (vname.clone(), vec![cs[row]]))
                .collect(),
            timing: self.timing,
            sense: self.sense,
        }
    }
}

impl Index<(usize, &str)> for Solution {
    type Output = Bias;

    /// Indexes a solution by `(row, variable_name)`.
    fn index(&self, index: (usize, &str)) -> &Self::Output {
        let (row, var_name) = index;
        &self.samples[var_name][row]
    }
}

impl Index<(usize, &String)> for Solution {
    type Output = Bias;

    /// Indexes a solution by `(row, variable_name)`.
    fn index(&self, index: (usize, &String)) -> &Self::Output {
        let (row, var_name) = index;
        &self.samples[var_name][row]
    }
}

impl Index<(usize, usize)> for Solution {
    type Output = Bias;

    /// Indexes a solution by `(row, column_position)`.
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row, col) = index;
        &self.samples[col][row]
    }
}
