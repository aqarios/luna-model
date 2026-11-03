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

impl Bounds {
    /// Returns the integer values these bounds admit, as bounds with integral endpoints.
    ///
    /// Fractional endpoints are rounded inward (lower up, upper down). Endpoints
    /// within `1e-9` of an integer are snapped to it first, so float noise like
    /// `0.29 * 100 = 28.999999999999996` keeps `29`. The result can be empty
    /// (`lower > upper`), e.g. for `0.2 <= x <= 0.8`.
    pub fn integer_domain(&self) -> Self {
        const SNAP_TOL: f64 = 1e-9;
        let round = |b: Bound, inward: fn(f64) -> f64| match b {
            Bound::Bounded(v) if (v - v.round()).abs() <= SNAP_TOL => Bound::Bounded(v.round()),
            Bound::Bounded(v) => Bound::Bounded(inward(v)),
            Bound::Unbounded => Bound::Unbounded,
        };
        Self {
            lower: round(self.lower, f64::ceil),
            upper: round(self.upper, f64::floor),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn domain(lower: f64, upper: f64) -> Bounds {
        Bounds::new(Bound::Bounded(lower), Bound::Bounded(upper)).integer_domain()
    }

    #[test]
    fn integer_domain_rounds_inward() {
        assert_eq!(domain(0.5, 3.7), domain(1.0, 3.0));
        assert_eq!(domain(-3.7, -0.5), domain(-3.0, -1.0));
    }

    #[test]
    fn integer_domain_snaps_float_noise() {
        assert_eq!(domain(0.0, 0.29 * 100.0), domain(0.0, 29.0));
    }

    #[test]
    fn integer_domain_can_be_empty() {
        let b = domain(0.2, 0.8);
        assert_eq!(b, domain(1.0, 0.0));
    }
}
