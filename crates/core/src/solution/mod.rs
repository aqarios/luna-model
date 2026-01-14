mod access;
mod col;
mod convenience;
mod creation;
mod filter;
mod io;
mod modification;
pub mod result;
mod sample;
mod samples;
mod src;
mod timing;

pub use col::{Assignment, Column};
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
#[derive(Debug, Clone, Default)]
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
    pub constraints: HashMap<String, Vec<bool>>,
    /// Boolean flag for each variable whether it's bounds are satisfied for each sample.
    /// variable_bounds[name].len() == samples[name].len() == n_samples
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
    pub fn with_sense(sense: Sense) -> Self {
        let mut out = Self::default();
        out.sense = sense;
        out
    }
}

impl ContentEquality for Solution {
    fn equal_contents(&self, other: &Self) -> bool {
        self == other
    }
}

impl PartialEq for Solution {
    fn eq(&self, other: &Self) -> bool {
        // eprintln!("A");
        for (cname, vs) in self.constraints.iter() {
            if let Some(otr_vs) = other.constraints.get(cname) {
                if vs != otr_vs {
                    return false;
                }
            } else {
                return false;
            }
        }

        // eprintln!("B");

        for (vname, vvals) in self.variable_bounds.iter() {
            if let Some(otr_vvals) = other.variable_bounds.get(vname) {
                if vvals != otr_vvals {
                    return false;
                }
            } else {
                return false;
            }
        }
        // dbg!(self.samples == other.samples);
        // dbg!(self.timing == other.timing);
        // dbg!(self.sense == other.sense);
        // dbg!(self.counts == other.counts);
        // dbg!(self.raw_energies == other.raw_energies);
        // dbg!(self.obj_values == other.obj_values);
        // dbg!(&self.feasible);
        // dbg!(&other.feasible);
        // dbg!(self.feasible == other.feasible);

        let feasible_eq = match (&self.feasible, &other.feasible) {
            (Some(slf), Some(otr)) => slf == otr,
            (Some(slf), None) => slf.is_empty(),
            (None, Some(otr)) => otr.is_empty(),
            (None, None) => true,
        };

        // variable_bounds: HashMap<String, Vec<bool>>,
        self.samples == other.samples
            && self.timing == other.timing
            // && self.n_samples == other.n_samples
            && self.sense == other.sense
            && self.counts == other.counts
            && self.raw_energies == other.raw_energies
            && self.obj_values == other.obj_values
            && feasible_eq
    }
}