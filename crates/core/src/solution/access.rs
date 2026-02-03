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
    pub fn n_samples(&self) -> usize {
        match self.samples.first() {
            Some((_, col)) => col.len(),
            None => 0,
        }
    }

    pub fn len(&self) -> usize {
        self.n_samples()
    }

    pub fn sample_len(&self) -> usize {
        self.samples.len()
    }

    pub fn value(&self, sample: usize, var: &str) -> Option<&Bias> {
        match sample >= self.len() {
            true => None,
            false => Some(&self[(sample, var)]),
        }
    }

    pub fn assignment(&self, sample: usize, var: &str) -> Assignment {
        self.samples[var].as_assignment(sample)
    }

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

    pub fn try_assignment_idx(&self, sample: usize, var: usize) -> LunaModelResult<Assignment> {
        if sample >= self.n_samples() {
            return Err(LunaModelError::IndexOutOfBounds(
                format!("{}, number of samples is {}", sample, self.n_samples()).into(),
            ));
        }
        if var >= self.samples.len() {
            return Err(LunaModelError::IndexOutOfBounds(
                format!("{}, sample length is {}", var, self.sample_len()).into(),
            ));
        }
        Ok(self.samples[var].as_assignment(sample))
    }

    pub fn result(&self, index: usize) -> Option<ResultView<'_>> {
        match index >= self.len() {
            true => None,
            false => Some((self, index).into()),
        }
    }

    pub fn results<'s>(&'s self) -> impl Iterator<Item = ResultView<'s>> {
        (0..self.len()).map(move |i| (self, i).into())
    }

    pub fn sample(&self, index: usize) -> Option<SampleView<'_>> {
        match index >= self.len() {
            true => None,
            false => Some((self, index).into()),
        }
    }

    pub fn samples<'s>(&'s self) -> impl Iterator<Item = SampleView<'s>> {
        (0..self.len()).map(move |i| (self, i).into())
    }

    pub fn variable_names(&self) -> Vec<String> {
        self.samples.keys().cloned().collect()
    }

    pub fn best(&self) -> Option<Vec<ResultView<'_>>> {
        match (&self.feasible, &self.obj_values) {
            (Some(f), Some(ov)) => {
                let target_map = ov.iter().zip(f).filter(|(_, f)| **f).map(|(a, _)| a);
                let target = *match self.sense {
                    Sense::Min => target_map.min_by(|&a, &b| a.total_cmp(b)).unwrap(),
                    Sense::Max => target_map.max_by(|&a, &b| a.total_cmp(b)).unwrap(),
                };
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
                    .map(|idx| ResultView::new(&self, *idx))
                    .collect();
                Some(views)
            }
            _ => None,
        }
    }

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
            timing: self.timing.clone(),
            sense: self.sense,
        }
    }
}

impl Index<(usize, &str)> for Solution {
    type Output = Bias;

    fn index(&self, index: (usize, &str)) -> &Self::Output {
        let (row, var_name) = index;
        &self.samples[var_name][row]
    }
}

impl Index<(usize, &String)> for Solution {
    type Output = Bias;

    fn index(&self, index: (usize, &String)) -> &Self::Output {
        let (row, var_name) = index;
        &self.samples[var_name][row]
    }
}

impl Index<(usize, usize)> for Solution {
    type Output = Bias;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row, col) = index;
        &self.samples[col][row]
    }
}
