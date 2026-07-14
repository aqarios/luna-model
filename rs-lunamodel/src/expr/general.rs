use crate::traits::{LmTryInto, ToPtrList};
use crate::{Error, Expression, Result, Status, TryDeepClone, TryIndex, VarRef};

impl Expression {
    pub fn deep_clone_many(exprs: &[&Expression]) -> Result<Vec<Expression>> {
        let Some(first) = exprs.first() else {
            return Ok(Vec::new());
        };
        let source = first.env()?;
        let source_id = source.id()?;
        let target = source.try_deep_clone(())?;
        exprs
            .iter()
            .map(|expr| {
                if expr.env()?.id()? != source_id {
                    return Err(Error {
                        status: Status::InvalidArgument,
                        message: "expressions belong to different environments".to_owned(),
                    });
                }
                expr.deep_clone(&target)
            })
            .collect()
    }

    /// Evaluates the expression against a single sample accessible by variable
    /// name. Dispatches into the C ABI; the summation lives in the host core.
    pub fn evaluate_sample<S>(&self, sample: &S) -> Result<f64>
    where
        for<'a> S: TryIndex<&'a str, Output = f64>,
    {
        let vars = self.complete_vars()?;
        let mut names: Vec<String> = Vec::with_capacity(vars.len());
        let mut values: Vec<f64> = Vec::with_capacity(vars.len());
        for var in &vars {
            let name = var.name()?;
            values.push(sample.try_index(name.as_str())?);
            names.push(name);
        }
        let names = names.as_slice().to_ptr_list()?;
        out!(expr_evaluate_sample(@self, names.as_ptr(), values.as_ptr(), values.len()))
    }

    /// Evaluates the expression across every sample row of `sol`.
    pub fn evaluate(&self, sol: &crate::Solution) -> Result<Vec<f64>> {
        out!(expr_evaluate(@self, @sol))
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

