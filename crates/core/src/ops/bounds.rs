//! Evaluation and helper operations for bounds.
use lunamodel_error::LunaModelResult;
use lunamodel_types::{Bias, Bound};
use lunamodel_utils::{float_ge, float_le, validate_tol};

use crate::bounds::Bounds;

impl Bounds {
    /// Returns whether `val` lies within the closed interval represented by these bounds.
    ///
    /// Unbounded sides are treated as always satisfied.
    pub fn evaluate(&self, val: Bias, tol: Option<f64>) -> LunaModelResult<bool> {
        let tol = validate_tol(tol)?;
        let lok = match self.lower {
            Bound::Unbounded => true,
            Bound::Bounded(bound) => float_ge(val, bound, tol),
        };
        let uok = match self.upper {
            Bound::Unbounded => true,
            Bound::Bounded(bound) => float_le(val, bound, tol),
        };
        Ok(lok && uok)
    }
}
