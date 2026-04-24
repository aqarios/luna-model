//! Deep-cloning helpers for expressions.

use lunamodel_error::{LunaModelError, LunaModelResult};

use super::Expression;
use crate::ArcEnv;

impl Expression {
    /// Clones the expression into a different environment wrapper.
    ///
    /// The algebraic storage itself is copied directly; only the environment
    /// handle is replaced. This is safe because the expression stores raw
    /// variable indices and expects the target environment to contain the
    /// corresponding variables.
    pub fn deep_clone(&self, env: ArcEnv) -> Self {
        let mut out = self.clone();
        out.env = env;
        out
    }

    /// Deep-clones many expressions into one newly cloned shared environment.
    ///
    /// All input expressions must currently refer to the same environment.
    pub fn deep_clone_many(exprs: &[&Expression]) -> LunaModelResult<Vec<Expression>> {
        if exprs.is_empty() {
            return Ok(Vec::new());
        }

        let old_env = &exprs[0].env;
        let new_env = old_env.deep_clone();

        let mut res = Vec::new();
        for expr in exprs {
            if !expr.env.read_arc().eq(&old_env.read_arc()) {
                return Err(LunaModelError::DifferentEnvironments);
            }
            res.push(expr.deep_clone(new_env.clone()));
        }

        Ok(res)
    }
}
