//! Column-oriented solution tables and related view types.

mod access;
mod col;
mod convenience;
mod creation;
mod filter;
mod merge;
mod modification;
mod random;
pub mod result;
pub mod sample;
mod samples;
mod src;
pub mod timing;

pub use col::{Assignment, ColElement, Column};
use indexmap::IndexMap;
use lunamodel_types::Sense;
pub use src::ValueSource;
use std::collections::HashMap;
pub use timing::{Timer, Timing};

use crate::traits::ContentEquality;

/// Column-oriented solution data for a model evaluation or solver result.
///
/// A [`Solution`] intentionally does not own or reference a [`crate::Model`] or
/// its [`crate::Environment`]. Instead, all variable-related data is keyed by
/// variable name. This makes a solution independent of environment-local
/// variable indices and therefore easier to merge, serialize, inspect, and move
/// between workflows that no longer have direct access to the original model.
///
/// The `samples` field stores data in column orientation:
///
/// ```text
/// variable name -> all values for that variable across samples
///
/// x -> [x0, x1, x2, ...]
/// y -> [y0, y1, y2, ...]
/// z -> [z0, z1, z2, ...]
/// ```
///
/// All per-sample metadata vectors, such as `counts`, `raw_energies`,
/// `obj_values`, and `feasible`, are expected to have the same length as each
/// stored column.
#[derive(Debug, Clone, Default)]
pub struct Solution {
    /// Column-oriented sample data keyed by variable name.
    ///
    /// Each [`Column`] contains all values for one variable across all samples.
    /// Every stored column is expected to have the same length.
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
    /// Per-constraint feasibility flags keyed by constraint name.
    ///
    /// Each vector is aligned with the stored samples and indicates whether the
    /// corresponding sample satisfies that named constraint.
    pub constraints: HashMap<String, Vec<bool>>,
    /// Boolean flag for each variable whether its bounds are satisfied for each sample.
    /// For every variable key, the corresponding vector length matches the number
    /// of stored samples.
    pub variable_bounds: HashMap<String, Vec<bool>>,
    // metadata
    /// Runtime metrics of the solution.
    pub timing: Option<Timing>,
    // /// Keeps track of the current number of unique samples.
    // pub n_samples: usize,
    /// The model's optimization sense the solution was created with.
    pub sense: Sense,
}

impl Solution {
    /// Create a default solution for a given [`Sense`].
    pub fn with_sense(sense: Sense) -> Self {
        Self {
            sense,
            ..Default::default()
        }
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self {
            samples: IndexMap::with_capacity(cap),
            ..Default::default()
        }
    }

    pub fn set_sense(mut self, sense: Sense) -> Self {
        self.sense = sense;
        self
    }
}

impl ContentEquality for Solution {
    fn equal_contents(&self, other: &Self) -> bool {
        self == other
    }
}

impl PartialEq for Solution {
    fn eq(&self, other: &Self) -> bool {
        for (cname, vs) in self.constraints.iter() {
            if let Some(otr_vs) = other.constraints.get(cname) {
                if vs != otr_vs {
                    return false;
                }
            } else {
                return false;
            }
        }

        for (vname, vvals) in self.variable_bounds.iter() {
            if let Some(otr_vvals) = other.variable_bounds.get(vname) {
                if vvals != otr_vvals {
                    return false;
                }
            } else {
                return false;
            }
        }
        let feasible_eq = match (&self.feasible, &other.feasible) {
            (Some(slf), Some(otr)) => slf == otr,
            (Some(slf), None) => slf.is_empty(),
            (None, Some(otr)) => otr.is_empty(),
            (None, None) => true,
        };

        self.samples == other.samples
            && self.timing == other.timing
            && self.sense == other.sense
            && self.counts == other.counts
            && self.raw_energies == other.raw_energies
            && self.obj_values == other.obj_values
            && feasible_eq
    }
}
