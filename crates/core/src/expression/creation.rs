use lunamodel_error::LunaModelResult;
use lunamodel_types::Bias;

use super::Expression;
use super::term::Linear;
use crate::ArcEnv;
use crate::prelude::Quadratic;

impl Expression {
    /// Creates a completely empty expression for an environment.
    ///
    /// This is the low-level constructor used when callers want to spell out the
    /// exact initial term storage instead of going through a convenience
    /// conversion.
    pub fn empty(env: ArcEnv) -> Self {
        Self {
            env,
            offset: Bias::default(),
            linear: Linear::default(),
            quadratic: None,
            higher_order: None,
        }
    }

    /// Creates a constant expression with the given offset.
    pub fn constant(env: ArcEnv, val: Bias) -> Self {
        let mut slf = Self::empty(env);
        slf.offset += val;
        slf
    }

    /// Builds an expression from a dense quadratic matrix.
    ///
    /// The matrix is interpreted as a full square `num_vars x num_vars` matrix in
    /// row-major order. Diagonal entries are folded into linear terms, and
    /// symmetric off-diagonal entries are summed into the upper-triangular
    /// quadratic storage used by LunaModel.
    pub fn from_dense_quadratic(
        dense: &[f64],
        num_vars: usize,
        offset: Option<Bias>,
        env: ArcEnv,
    ) -> LunaModelResult<Self> {
        let mut expr = match offset {
            Some(offset) => Self::constant(env, offset),
            None => Self::empty(env),
        };
        expr.quadratic = Some(Quadratic::default());
        for u in 0..num_vars {
            // diagonal
            expr.linear += (u as u32, dense[u * (num_vars + 1)]);
            // off-diagonal
            for v in (u + 1)..num_vars {
                let qbias = dense[u * num_vars + v] + dense[v * num_vars + u];
                *expr.quadratic.as_mut().unwrap() += (u as u32, v as u32, qbias);
            }
        }
        Ok(expr)
    }
}
