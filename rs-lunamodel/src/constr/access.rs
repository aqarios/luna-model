use lunamodel_types::Comparator;

use crate::{Constraint, Expression, Result, TryIndex, traits::ToPtrList};

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
        let vars = self.lhs()?.complete_vars()?;
        let mut names = Vec::with_capacity(vars.len());
        let mut values = Vec::with_capacity(vars.len());
        for var in vars {
            let name = var.name()?;
            values.push(sample.try_index(name.as_str())?);
            names.push(name);
        }
        let names = names.as_slice().to_ptr_list()?;
        out!(constr_evaluate_sample(
            @self,
            names.as_ptr(),
            values.as_ptr(),
            values.len(),
            ?tol
        ))
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
