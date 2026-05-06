//! Comparator and constraint-type enums shared across the workspace.
use lunamodel_error::LunaModelResult;
use lunamodel_utils::{float_eq, float_ge, float_le, validate_tol};
use strum_macros::Display;

use crate::Bias;

/// Comparison operators used to define constraints.
///
/// This enum represents the logical relation between the left-hand side (LHS)
/// and the right-hand side (RHS) of a constraint.
#[derive(Debug, Copy, Clone, PartialEq, Display, Eq, Hash)]
pub enum Comparator {
    /// The Equality comparison (==) for a constraint where LHS == RHS.
    #[strum(to_string = "==")]
    Eq,
    /// The Less-than or equal comparison (<=) for a constraint where LHS <= RHS.
    #[strum(to_string = "<=")]
    Le,
    /// The Greater-than or equal comparison (>=) for a constraint where LHS >= RHS.
    #[strum(to_string = ">=")]
    Ge,
}

impl Comparator {
    /// Evaluates the comparator on `lhs` and `rhs`.
    ///
    /// Floating-point comparisons use `tol` to absorb small numerical drift.
    /// Equality accepts values whose absolute difference is within
    /// `tol + f64::EPSILON * max(abs(lhs), abs(rhs), 1.0)`. Inequalities allow
    /// the same tolerance in the violated direction. If `tol` is `None`,
    /// the default tolerance of `1e-6` is used.
    pub fn evaluate(self, lhs: Bias, rhs: Bias, tol: Option<f64>) -> LunaModelResult<bool> {
        let tol = validate_tol(tol)?;
        Ok(match self {
            Self::Eq => float_eq(lhs, rhs, tol),
            Self::Le => float_le(lhs, rhs, tol),
            Self::Ge => float_ge(lhs, rhs, tol),
        })
    }
}

#[cfg(test)]
mod tests {
    use lunamodel_error::LunaModelResult;

    use super::Comparator;

    #[test]
    fn equality_allows_default_tolerance() -> LunaModelResult<()> {
        assert!(Comparator::Eq.evaluate(1.0 + 1e-7, 1.0, None)?);
        assert!(!Comparator::Eq.evaluate(1.0 + 1e-5, 1.0, None)?);
        Ok(())
    }

    #[test]
    fn less_equal_allows_default_tolerance() -> LunaModelResult<()> {
        assert!(Comparator::Le.evaluate(1.0 + 1e-7, 1.0, None)?);
        assert!(!Comparator::Le.evaluate(1.0 + 1e-5, 1.0, None)?);
        Ok(())
    }

    #[test]
    fn greater_equal_allows_default_tolerance() -> LunaModelResult<()> {
        assert!(Comparator::Ge.evaluate(1.0 - 1e-7, 1.0, None)?);
        assert!(!Comparator::Ge.evaluate(1.0 - 1e-5, 1.0, None)?);
        Ok(())
    }

    #[test]
    fn evaluate_rejects_invalid_tolerance() {
        for tol in [
            Some(-f64::EPSILON),
            Some(1.0),
            Some(f64::NAN),
            Some(f64::INFINITY),
        ] {
            assert!(Comparator::Eq.evaluate(1.0, 1.0, tol).is_err());
        }
    }
}
