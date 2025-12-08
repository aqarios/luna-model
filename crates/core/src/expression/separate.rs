use super::Expression;
use crate::variable::VarRef;

impl Expression {
    pub fn separate(&self, vars: &[VarRef]) -> (Expression, Expression) {
        _ = vars;
        todo!()
    }
}
