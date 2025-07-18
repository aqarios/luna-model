use std::fmt::Display;

use crate::{
    core::{solution::sample::SampleOwned, writer::SolutionWriter, Sample},
    types::Bias,
};

#[derive(Debug)]
pub struct OwnedResult {
    /// The vector of variable assignments.
    pub sample: SampleOwned,
    /// The objective value computed from an AqModel. If not present, a raw value from the solver
    /// may be used. None, if none of these are present.
    pub obj_value: Option<Bias>,
    /// Boolean flag for each single constraint whether it's satisfied.
    pub constraint_satisfaction: Option<Vec<bool>>,
    /// Boolean flag for each variable bounds whether it's satisfied.
    pub variable_bounds_satisfaction: Option<Vec<bool>>,
    /// Whether all constraints are satisfied.
    pub feasible: Option<bool>,
}

impl OwnedResult {
    pub fn new(
        sample: SampleOwned,
        objective_value: Bias,
        constraint_satisfaction: Vec<bool>,
        variable_bounds_satisfaction: Vec<bool>,
        feasible: bool,
    ) -> Self {
        Self {
            sample,
            obj_value: Some(objective_value),
            constraint_satisfaction: Some(constraint_satisfaction),
            variable_bounds_satisfaction: Some(variable_bounds_satisfaction),
            feasible: Some(feasible),
        }
    }
}

impl Display for OwnedResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = SolutionWriter::new()
            .write_sample(&Sample::Owned(self.sample.clone()))
            .to_string();
        f.write_str(&s)
    }
}
