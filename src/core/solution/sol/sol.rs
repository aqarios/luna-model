use super::column::Column;
use crate::{
    core::{writer::SolutionWriter, ContentEquality, Sense, Timing},
    types::Bias,
};

/// The solutions object for AQMs. It doesn't have any knowledge about the corresponding AQM or
/// about the environment the model was created in. Instead, for each sample, we expect the indices
/// of the solution to be aligned with the variable indices of the model's environment.
#[derive(Debug, Clone, Default)]
pub struct Solution {
    /// A collection of samples. Each inner vec corresponds to all assignments to a single variable
    /// across different samples. `samples.len()` can be expected to always correspond exactly to
    /// the number of results available in the solution.
    pub samples: Vec<Column>,
    /// How often each result occurs in the solution. `counts.len()` can be expected to
    /// always be equal to `samples.len()`
    pub counts: Vec<usize>,
    /// Objetive values as computed by the solver. May be empty if the solver does not provide
    /// energies in its solution format. May be different from `obj_values`, e.g., because an offset
    /// was neglected, or the AQM was transformed before being solved.
    pub raw_energies: Option<Vec<Bias>>,
    /// Objetive values as computed by the corresponding AQM. May be empty for solutions that
    /// haven't yet been evaluated.
    pub obj_values: Option<Vec<Bias>>,
    /// Boolean flag for each single constraint whether it's satisfied. Each inner vec corresponds
    /// to one sample, i.e., `constraints[i]` corresponds to `samples[i]`. May be empty for
    /// solutions that haven't yet been evaluated.
    pub constraints: Option<Vec<Vec<bool>>>,
    /// Boolean flag for each sample whether it's feasible, i.e., whether all bounds are satisfied.
    /// May be empty for solutions that haven't yet been evaluated.
    pub variable_bounds: Option<Vec<Vec<bool>>>,
    /// Boolean flag for each sample whether it's feasible, i.e., whether all constraints are
    /// satisfied. In other words, `feasible[i]` iff. `all(constraints[i])`. May be empty for
    /// solutions that haven't yet been evaluated.
    pub feasible: Option<Vec<bool>>,
    /// Metadata that may be useful for explaining why a constraint is not satisfied, e.g., the eval
    /// of a lhs.
    pub best_sample_idx: Option<usize>,
    /// Runtime metrics of the solution.
    pub timing: Option<Timing>,
    /// Keeps track of the current number of unique samples.
    pub n_samples: usize,
    /// The names of all variables present in the solution.
    pub variable_names: Vec<String>,
    /// The model's optimization sense the solution was created with.
    pub sense: Sense,
}

impl Solution {
    pub fn with_sense(sense: Sense) -> Self {
        let mut out = Self::default();
        out.sense = sense;
        out
    }
}

impl std::fmt::Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = SolutionWriter::new().write_solution(&self).to_string();
        f.write_str(&s)
    }
}

impl ContentEquality for Solution {
    fn is_equal_contents(&self, other: &Self) -> bool {
        self == other
    }
}

impl PartialEq for Solution {
    fn eq(&self, other: &Self) -> bool {
        self.samples == other.samples
            && self.counts == other.counts
            && self.obj_values == other.obj_values
            && self.raw_energies == other.raw_energies
            && self.constraints == other.constraints
            && self.variable_bounds == other.variable_bounds
            && self.feasible == other.feasible
            && self.best_sample_idx == other.best_sample_idx
            && self.timing == other.timing
            && self.n_samples == other.n_samples
            && self.variable_names == other.variable_names
    }
}
