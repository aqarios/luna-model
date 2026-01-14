use lunamodel_error::LunaModelResult;
use lunamodel_types::Bias;

use crate::variable::VarRef;

impl VarRef {
    pub fn evaluate(&self, value: Bias) -> LunaModelResult<bool> {
        self.check_living()?;
        let bounds = self.bounds()?;
        Ok(bounds.evaluate(value))
    }
}
