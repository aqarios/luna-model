use lunamodel_error::LunaModelResult;
use lunamodel_types::Bias;

use crate::{Expression, ops::LmAddAssign, variable::VarRef};

impl Expression {
    /// Filter an expression based on some condition for each item.
    /// Resulting in a new expression.
    pub fn filter<F>(&self, f: F) -> LunaModelResult<Self>
    where
        F: Fn(&Vec<VarRef>, Bias) -> LunaModelResult<bool>,
    {
        let mut out = Expression::empty(self.env.clone());
        for (vars, bias) in self.items() {
            if f(&vars, bias)? {
                out.add_assign((vars.as_slice(), bias))?;
            }
        }
        Ok(out)
    }
}
