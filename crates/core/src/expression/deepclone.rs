use lunamodel_error::{LunaModelError, LunaModelResult};

use super::Expression;
use crate::ArcEnv;

impl Expression {
    pub fn deep_clone(&self, env: ArcEnv) -> Self {
        let mut out = self.clone();
        out.env = env;
        out
    }

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
