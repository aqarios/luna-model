mod access;
mod col;
mod convenience;
mod creation;
mod filter;
mod modification;
pub mod result;
mod io;
mod sample;
mod samples;
mod src;
mod timing;

pub use col::Column;
use hashbrown::HashMap;
use indexmap::IndexMap;
use lunamodel_types::Sense;
pub use src::ValueSource;
pub use timing::{Timer, Timing};

use crate::traits::ContentEquality;

/// The solutions object for Models. It doesn't have any knowledge about the corresponding AQM or
/// about the environment the model was created in. Instead, for each sample, we expect the indices
/// of the solution to be aligned with the variable indices of the model's environment.
///
///   x y z . .
/// [ . . . . . ] count0
/// [ c . . . . ] count1
/// [ o . . . . ] count2
/// [ l . . . . ]
/// [ . . . . . ]
///
/// counts.len() == samples[?].len()
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Solution {
    /// A collection of samples. The data is stored in column orientation. Each column contains all
    /// values for the variable over all samples. The number of samples is equal to the number of
    /// elements in the [Column]s.
    pub samples: IndexMap<String, Column>,
    // pub samples: Samples,
    /// How often each sample occurs in the solution. The counts length matches the number of
    /// samples, i.e., it matches the length of the [Column]s in the samples.
    pub counts: Vec<usize>,
    /// Objetive values as computed by the solver. May be empty if the solver does not provide
    /// energies in its solution format. May be different from `obj_values`, e.g., because an offset
    /// was neglected, or the Model was transformed before being solved.
    pub raw_energies: Option<Vec<f64>>,
    /// Objetive values as computed by the corresponding Model. May be empty for solutions that
    /// haven't yet been evaluated.
    pub obj_values: Option<Vec<f64>>,
    /// Boolean flag for each sample whether it's feasible, i.e., whether all constraints are
    /// satisfied. In other words, `feasible[i]` iff. `all(constraints[i])`. May be empty for
    /// solutions that haven't yet been evaluated.
    pub feasible: Option<Vec<bool>>,
    // constr
    /// Boolean flag for each single constraint whether it's satisfied. Each inner map corresponds
    /// to one sample, i.e., `constraints[name]` corresponds to `samples[?].len()`. May be empty for
    /// solutions that haven't yet been evaluated.
    pub constraints: Vec<HashMap<String, bool>>,
    /// Boolean flag for each variable whether it's bounds are satisfied for each sample.
    /// variable_bounds[name].len() == samples[name].len() == n_samples
    pub variable_bounds: HashMap<String, Vec<bool>>,
    // metadata
    /// Runtime metrics of the solution.
    pub timing: Option<Timing>,
    /// Keeps track of the current number of unique samples.
    pub n_samples: usize,
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

impl ContentEquality for Solution {
    fn is_equal_contents(&self, other: &Self) -> bool {
        self == other
    }
}
