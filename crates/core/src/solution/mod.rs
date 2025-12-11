mod col;
mod src;
mod timing;

use hashbrown::HashMap;

pub use col::Column;
use lunamodel_types::Sense;
pub use src::ValueSource;
pub use timing::Timing;

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
    pub samples: HashMap<String, Column>,
    // per sample
    pub counts: Vec<usize>,
    pub raw_energies: Option<Vec<usize>>,
    pub obj_values: Option<Vec<usize>>,
    pub feasible: Option<Vec<bool>>,
    // constr
    pub constraints: Vec<HashMap<String, bool>>,
    pub variable_bounds: Vec<HashMap<String, bool>>,
    // metadata
    pub timing: Option<Timing>,
    pub n_samples: Option<usize>,
    pub sense: Sense,
}

impl Solution {
    pub fn with_sense(sense: Sense) -> Self {
        let mut out = Self::default();
        out.sense = sense;
        out
    }
}
