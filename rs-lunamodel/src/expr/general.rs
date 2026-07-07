use lunamodel_cext_vtable::LmVref;

use crate::{Expression, Result, VarRef};

impl Expression {
    /// Replaces every occurrence of `target` with `replacement`, preserving the
    /// environment.
    pub fn substitute(&self, target: &VarRef, replacement: &Expression) -> Result<Expression> {
        out!(expr_substitute(@self, @target, @replacement), |o| Ok(Expression::from_raw(o)))
    }

    /// Splits the expression by `vrefs` into two: the first holds every
    /// contribution involving at least one variable in `vrefs`, the second holds
    /// the remainder.
    pub fn separate(&self, vrefs: &[VarRef]) -> Result<(Expression, Expression)> {
        let ptrs: Vec<*const LmVref> = vrefs.iter().map(|v| v.as_ptr()).collect();
        let mut a = std::ptr::null_mut();
        let mut b = std::ptr::null_mut();
        check!(expr_separate(@self, ptrs.as_ptr(), ptrs.len(), &mut a, &mut b))?;
        Ok((Expression::from_raw(a), Expression::from_raw(b)))
    }
}
