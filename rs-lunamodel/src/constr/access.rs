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
        let lhs = self.lhs()?.evaluate_sample(sample)?;
        evaluate(self.comparator()?, lhs, self.rhs()?, tol)
    }

    pub fn evaluate_sample_quick(&self, lookup: &[f64], tol: Option<f64>) -> Result<bool> {
        let lhs = self.lhs()?.evaluate_sample_quick(lookup)?;
        evaluate(self.comparator()?, lhs, self.rhs()?, tol)
    }
}

fn evaluate(comparator: Comparator, lhs: f64, rhs: f64, tol: Option<f64>) -> Result<bool> {
    let tol = lunamodel_utils::validate_tol(tol).map_err(|err| crate::Error {
        status: crate::Status::InvalidArgument,
        message: err.to_string(),
    })?;
    Ok(match comparator {
        Comparator::Eq => lunamodel_utils::float_eq(lhs, rhs, tol),
        Comparator::Le => lunamodel_utils::float_le(lhs, rhs, tol),
        Comparator::Ge => lunamodel_utils::float_ge(lhs, rhs, tol),
    })
}
