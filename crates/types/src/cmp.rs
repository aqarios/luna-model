//! Comparator and constraint-type enums shared across the workspace.
use lunamodel_utils::defaults::DEFAULT_TOL;
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
    pub fn evaluate(self, lhs: Bias, rhs: Bias, tol: Option<f64>) -> bool {
        let tol = tol.unwrap_or_else(|| DEFAULT_TOL);
        match self {
            Self::Eq => float_eq(lhs, rhs, tol),
            Self::Le => float_le(lhs, rhs, tol),
            Self::Ge => float_ge(lhs, rhs, tol),
        }
    }
}

fn comparison_tolerance(lhs: Bias, rhs: Bias, tol: f64) -> Bias {
    tol + Bias::EPSILON * lhs.abs().max(rhs.abs()).max(1.0)
}

fn float_eq(lhs: Bias, rhs: Bias, tol: f64) -> bool {
    if lhs == rhs {
        return true;
    }
    if !lhs.is_finite() || !rhs.is_finite() {
        return false;
    }

    (lhs - rhs).abs() <= comparison_tolerance(lhs, rhs, tol)
}

fn float_le(lhs: Bias, rhs: Bias, tol: f64) -> bool {
    if lhs <= rhs {
        return true;
    }
    if !lhs.is_finite() || !rhs.is_finite() {
        return false;
    }

    lhs - rhs <= comparison_tolerance(lhs, rhs, tol)
}

fn float_ge(lhs: Bias, rhs: Bias, tol: f64) -> bool {
    if lhs >= rhs {
        return true;
    }
    if !lhs.is_finite() || !rhs.is_finite() {
        return false;
    }

    rhs - lhs <= comparison_tolerance(lhs, rhs, tol)
}

#[cfg(test)]
mod tests {
    use super::Comparator;

    #[test]
    fn equality_allows_default_tolerance() {
        assert!(Comparator::Eq.evaluate(1.0 + 1e-7, 1.0, None));
        assert!(!Comparator::Eq.evaluate(1.0 + 1e-5, 1.0, None));
    }

    #[test]
    fn less_equal_allows_default_tolerance() {
        assert!(Comparator::Le.evaluate(1.0 + 1e-7, 1.0, None));
        assert!(!Comparator::Le.evaluate(1.0 + 1e-5, 1.0, None));
    }

    #[test]
    fn greater_equal_allows_default_tolerance() {
        assert!(Comparator::Ge.evaluate(1.0 - 1e-7, 1.0, None));
        assert!(!Comparator::Ge.evaluate(1.0 - 1e-5, 1.0, None));
    }
}
