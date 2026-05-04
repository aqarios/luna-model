use std::fmt::Debug;

use lunamodel_error::{LunaModelError, LunaModelResult};
use num::{NumCast, ToPrimitive};

use super::defaults::DEFAULT_TOL;

/// Resolves and validates a user-provided numeric tolerance.
///
/// When `tol` is `None`, this returns [`DEFAULT_TOL`]. Otherwise, the provided
/// value must be finite and satisfy `0.0 <= tol < 1.0`; invalid values return
/// [`LunaModelError::InvalidTolerance`].
pub fn validate_tol(tol: Option<f64>) -> LunaModelResult<f64> {
    let tol = tol.unwrap_or(DEFAULT_TOL);
    if !tol.is_finite() || !(0.0..1.0).contains(&tol) {
        return Err(LunaModelError::InvalidTolerance(
            "tol must be in [0.0, 1.0).".into(),
        ));
    }
    Ok(tol)
}

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
    let tol = validate_tol(tol)?;
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
