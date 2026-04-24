//! Small utility helpers shared across the core crate.

use core::result::Result::Ok;
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
    if !tol.is_finite() || !(0.0..1.0).contains(&tol) {
        return Err(LunaModelError::InvalidTolerance(
            "tol must be in [0.0, 1.0).".into(),
        ));
    }
    let Some(v) = value.to_f64() else {
        return Ok(None);
    };
    if !v.is_finite() {
        return Ok(None);
    }

    let r = v.round();
    let diff = (v - r).abs();
    // Widen tol by v's ulp so values at the edge of f64 precision still round
    // cleanly when tol is small or zero.
    let eff_tol = tol + f64::EPSILON * v.abs().max(1.0);

    if diff <= eff_tol {
        Ok(NumCast::from(r))
    } else {
        Ok(None)
    }
}
