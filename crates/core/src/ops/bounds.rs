//! Evaluation and helper operations for bounds.

use lunamodel_types::{Bias, Bound};

use crate::bounds::Bounds;

impl Bounds {
    /// Returns whether `val` lies within the closed interval represented by these bounds.
    ///
    /// Unbounded sides are treated as always satisfied.
    pub fn evaluate(&self, val: Bias) -> bool {
        let lok = match self.lower {
            Bound::Unbounded => true,
            Bound::Bounded(bound) => val >= bound,
        };
        let uok = match self.upper {
            Bound::Unbounded => true,
            Bound::Bounded(bound) => val <= bound,
        };
        lok && uok
    }
}
