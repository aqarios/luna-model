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

#[cfg(test)]
mod tests {
    use super::{cast_near_integral, validate_tol};

    #[test]
    fn validate_tol_accepts_default_and_valid_bounds() {
        assert_eq!(validate_tol(None).unwrap(), 1e-6);
        assert_eq!(validate_tol(Some(0.0)).unwrap(), 0.0);
        assert_eq!(validate_tol(Some(0.999_999)).unwrap(), 0.999_999);
    }

    #[test]
    fn validate_tol_rejects_invalid_values() {
        for tol in [
            Some(-f64::EPSILON),
            Some(1.0),
            Some(f64::NAN),
            Some(f64::INFINITY),
        ] {
            assert!(validate_tol(tol).is_err());
        }
    }

    #[test]
    fn cast_near_integral_accepts_values_within_tolerance() {
        assert_eq!(
            cast_near_integral::<i64, _>(2.999_999_9, Some(1e-6)).unwrap(),
            Some(3)
        );
        assert_eq!(
            cast_near_integral::<i64, _>(-1.999_999_9, Some(1e-6)).unwrap(),
            Some(-2)
        );
    }

    #[test]
    fn cast_near_integral_rejects_values_outside_tolerance() {
        assert_eq!(
            cast_near_integral::<i64, _>(2.99, Some(1e-6)).unwrap(),
            None
        );
    }

    #[test]
    fn cast_near_integral_handles_non_finite_values_without_casting() {
        assert_eq!(cast_near_integral::<i64, _>(f64::NAN, None).unwrap(), None);
        assert_eq!(
            cast_near_integral::<i64, _>(f64::INFINITY, None).unwrap(),
            None
        );
    }

    #[test]
    fn cast_near_integral_returns_none_when_target_type_overflows() {
        let too_large_for_u8 = f64::from(u8::MAX) + 1.0;
        assert_eq!(
            cast_near_integral::<u8, _>(too_large_for_u8, None).unwrap(),
            None
        );
    }

    #[test]
    fn cast_near_integral_rejects_invalid_tolerance() {
        assert!(cast_near_integral::<i64, _>(1.0, Some(1.0)).is_err());
    }
}
