//! Small utility helpers shared across the core crate.

use std::{fmt::Debug, ops::Mul};

use lunamodel_error::{LunaModelError, LunaModelResult};
use num::{NumCast, ToPrimitive};

use crate::prelude::{Expression, VarRef};

impl From<VarRef> for Expression {
    /// Promotes a variable reference into the equivalent one-term expression.
    ///
    /// This goes through the regular multiplication path instead of manually
    /// constructing linear storage so that all environment-sensitive checks stay
    /// centralized in one place.
    fn from(var: VarRef) -> Self {
        Expression::constant(var.env.clone(), 1.0)
            .mul(&var)
            .expect("the environment changed during cloning")
    }
}

const DEFAULT_TOL: f64 = 1e-6;

/// Casts a numeric value to an integral target type when it is sufficiently close.
///
/// This is used when external formats or floating-point workflows conceptually
/// carry integer-valued data but represent it as `f64`. Rather than requiring an
/// exact binary floating-point match, the function accepts values within a small
/// tolerance of the nearest integer.
///
/// Returns `Ok(None)` when the value is non-finite, not representable as `f64`,
/// or too far from an integer.
pub fn cast_near_integral<T: NumCast, N: ToPrimitive + Copy + Debug>(
    value: N,
    tol: Option<f64>,
) -> LunaModelResult<Option<T>> {
    let tol = tol.unwrap_or(DEFAULT_TOL);
    if tol <= 0.0 || tol > 1.0 {
        return Err(LunaModelError::Internal(
            "tol must be in [0.0, 1.0).".into(),
        ));
    }
    if !tol.is_finite() {
        return Ok(None);
    }

    if let Some(v) = value.to_f64() {
        if !v.is_finite() {
            return Ok(None);
        }

        let r = v.round();
        let diff = (v - r).abs();

        // Combined tolerance: caller-provided + machine-precision floor
        let eff_tol = tol + f64::EPSILON * v.abs().max(1.0);

        if diff <= eff_tol {
            Ok(NumCast::from(r))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}
