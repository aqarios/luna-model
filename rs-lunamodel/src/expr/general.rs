use crate::traits::LmTryInto;
use crate::{Expression, Result, TryIndex, VarRef};

impl Expression {
    pub fn deep_clone_many(exprs: &[&Expression]) -> Result<Vec<Expression>> {
        let inputs: Vec<*const lunamodel_cext_vtable::LmExpr> =
            exprs.iter().map(|expr| expr.as_ptr()).collect();
        let mut outputs = vec![std::ptr::null_mut(); exprs.len()];
        check!(expr_deep_clone_many(
            inputs.as_ptr(),
            inputs.len(),
            outputs.as_mut_ptr()
        ))?;
        outputs.into_iter().map(LmTryInto::lm_try_into).collect()
    }

    pub fn evaluate_sample_quick(&self, lookup: &[f64]) -> Result<f64> {
        out!(expr_evaluate_sample_quick(
            @self,
            lookup.as_ptr(),
            lookup.len()
        ))
    }

    pub fn evaluate_sample<S>(&self, sample: &S) -> Result<f64>
    where
        for<'a> S: TryIndex<&'a str, Output = f64>,
    {
        let env = self.env()?;
        let mut lookup = vec![0.0; env.len()?];
        for var in self.complete_vars()? {
            let idx = var.id()? as usize;
            let name = var.name()?;
            lookup[idx] = sample.try_index(name.as_str())?;
        }
        self.evaluate_sample_quick(&lookup)
    }

    pub fn evaluate_sampleset<'a, S, I>(&self, samples: I) -> Result<Vec<f64>>
    where
        for<'s> S: TryIndex<&'s str, Output = f64> + 'a,
        I: IntoIterator<Item = &'a S>,
    {
        samples
            .into_iter()
            .map(|sample| self.evaluate_sample(sample))
            .collect()
    }

    /// Replaces every occurrence of `target` with `replacement`, preserving the
    /// environment.
    pub fn substitute(&self, target: &VarRef, replacement: &Expression) -> Result<Expression> {
        out!(expr_substitute(@self, @target, @replacement))
    }

    /// Splits the expression by `vrefs` into two: the first holds every
    /// contribution involving at least one variable in `vrefs`, the second holds
    /// the remainder.
    pub fn separate(&self, vrefs: &[VarRef]) -> Result<(Expression, Expression)> {
        let (mut a, mut b) = (std::ptr::null_mut(), std::ptr::null_mut());
        check!(expr_separate(@self, *vrefs, &mut a, &mut b))?;
        let a = a.lm_try_into()?;
        let b = b.lm_try_into()?;
        Ok((a, b))
    }
}
