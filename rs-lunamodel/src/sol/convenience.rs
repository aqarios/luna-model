use lunamodel_types::ValueSource;

use crate::{Model, Result, Solution};

impl Solution {
    pub fn merge_many(
        solutions: &[Solution],
        model: Option<&Model>,
        tol: Option<f64>,
    ) -> Result<Solution> {
        out!(sol_merge_many(*solutions, ?@model, ?tol))
    }

    pub fn cvar(&self, alpha: f64, toggle: Option<ValueSource>) -> Result<f64> {
        out!(sol_cvar(@self, alpha, ?~toggle))
    }

    pub fn expectation_value(&self, toggle: Option<ValueSource>) -> Result<f64> {
        out!(sol_expectation_value(@self, ?~toggle))
    }

    pub fn temperature_weighted(&self, beta: f64, toggle: Option<ValueSource>) -> Result<f64> {
        out!(sol_temperature_weighted(@self, beta, ?~toggle))
    }

    pub fn feasibility_ratio(&self) -> Result<f64> {
        out!(sol_feasibility_ratio(@self))
    }

    pub fn highest_constraint_violations(&self) -> Result<Option<String>> {
        out_opt!(sol_highest_constraint_violations(@self))
    }
}
