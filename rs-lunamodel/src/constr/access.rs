use lunamodel_types::Comparator;

use crate::{Constraint, Expression, Result, TryIndex};

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

    pub fn evaluate_sample<S>(&self, sample: &S, tol: Option<f64>) -> Result<bool>
    where
        for<'a> S: TryIndex<&'a str, Output = f64>,
    {
        let lhs = self.lhs()?;
        let env = lhs.env()?;
        let mut lookup = vec![0.0; env.len()?];
        for var in lhs.complete_vars()? {
            lookup[var.id()? as usize] = sample.try_index(var.name()?.as_str())?;
        }
        self.evaluate_sample_quick(&lookup, tol)
    }

    pub fn evaluate_sample_quick(&self, lookup: &[f64], tol: Option<f64>) -> Result<bool> {
        out!(constr_evaluate_sample_quick(
            @self,
            lookup.as_ptr(),
            lookup.len(),
            ?tol
        ))
    }
}
