use indexmap::IndexMap;
use lunamodel_types::Sense;

use crate::{Assignment, Column, Result, ResultView, SampleView, Solution, Timing};

impl Solution {
    pub fn samples(&self) -> Result<IndexMap<String, Column>> {
        out!(sol_samples(@self))
    }

    pub fn counts(&self) -> Result<Vec<usize>> {
        out_slice!(sol_counts(@self))
    }

    pub fn raw_energies(&self) -> Result<Vec<f64>> {
        out_slice!(sol_raw_energies(@self))
    }

    pub fn obj_values(&self) -> Result<Vec<f64>> {
        out_slice!(sol_obj_values(@self))
    }

    pub fn feasible(&self) -> Result<Vec<bool>> {
        out_slice!(sol_feasible(@self))
    }

    pub fn timing(&self) -> Result<Timing> {
        out!(sol_timing(@self))
    }

    pub fn sense(&self) -> Result<Sense> {
        out!(sol_sense(@self))
    }

    pub fn n_samples(&self) -> Result<usize> {
        out!(sol_n_samples(@self))
    }

    pub fn len(&self) -> Result<usize> {
        out!(sol_len(@self))
    }

    pub fn variable_names(&self) -> Result<Vec<String>> {
        out!(sol_variable_names(@self))
    }

    pub fn is_empty(&self) -> Result<bool> {
        out!(sol_is_empty(@self))
    }

    pub fn sample_len(&self) -> Result<usize> {
        out!(sol_sample_len(@self))
    }

    pub fn value(&self, sample: usize, var: &str) -> Result<f64> {
        out!(sol_value(@self, sample, !var))
    }

    pub fn assignment(&self, sample: usize, var: &str) -> Result<Assignment> {
        out!(sol_assignment(@self, sample, !var))
    }

    pub fn assignment_idx(&self, sample: usize, var: usize) -> Result<Assignment> {
        out!(sol_assignment_idx(@self, sample, var))
    }

    pub fn result(&self, index: usize) -> Result<ResultView> {
        out!(sol_result(@self, index))
    }

    pub fn results(&self) -> Result<Vec<ResultView>> {
        out!(sol_results(@self))
    }

    pub fn sample_view(&self, index: usize) -> Result<SampleView> {
        out!(sol_sample_view(@self, index))
    }

    pub fn sample_views(&self) -> Result<Vec<SampleView>> {
        out!(sol_sample_views(@self))
    }

    pub fn best(&self) -> Result<Option<Vec<ResultView>>> {
        out_opt!(sol_best(@self))
    }

    pub fn extract(&self, row: usize) -> Result<Self> {
        out!(sol_extract(@self, row))
    }
}
