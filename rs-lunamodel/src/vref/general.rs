use super::VarRef;
use crate::traits::LmTryInto;

use crate::Result;
use crate::variable::Variable;

impl LmTryInto<Variable> for VarRef {
    fn lm_try_into(self) -> Result<Variable> {
        out!(vref_to_var(@self), |o| Ok(Variable::from_raw(o)))
    }
}
