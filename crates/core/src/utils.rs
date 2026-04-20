use std::{fmt::Debug, ops::Mul};

use lunamodel_error::{LunaModelError, LunaModelResult};
use num::{NumCast, ToPrimitive};

use crate::prelude::{Expression, VarRef};

impl From<VarRef> for Expression {
    fn from(var: VarRef) -> Self {
        Expression::constant(var.env.clone(), 1.0)
            .mul(&var)
            .expect("the environment changed during cloning")
    }
}

const DEFAULT_TOL: f64 = 1e-6;

pub fn cast_near_integral<T: NumCast, N: ToPrimitive + Copy + Debug>(
    value: N,
    tol: Option<f64>,
) -> LunaModelResult<Option<T>> {
    let tol = tol.unwrap_or_else(|| DEFAULT_TOL);
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
