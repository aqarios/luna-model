use lunamodel_types::Comparator;

use crate::{Constraint, Expression, Result};

impl Constraint {
    pub fn lhs(&self) -> Result<Expression> {
        out!(constr_lhs(@self))
    }

    pub fn rhs(&self) -> Result<f64> {
        out!(constr_rhs(@self))
    }

    pub fn comparator(&self) -> Result<Comparator> {
        out!(constr_comparator(@self))
    }

    pub fn name(&self) -> Result<String> {
        out!(constr_name(@self))
    }

    pub fn auto_name(&self) -> Result<bool> {
        out!(constr_auto_name(@self))
    }
}
