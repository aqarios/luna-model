use crate::traits::LmTryInto;
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

    pub fn evaluate_sample_quick(&self, lookup: &[f64]) -> Result<f64> {
        let mut value = self.offset()?;
        for (idx, bias) in self.raw_linear_items()? {
            value += lookup_value(lookup, idx)? * bias;
        }
        for (u, v, bias) in self.raw_quadratic_items()? {
            value += lookup_value(lookup, u)? * lookup_value(lookup, v)? * bias;
        }
        for (indices, bias) in self.raw_higher_order_items()? {
            let contribution = indices
                .into_iter()
                .try_fold(bias, |acc, idx| Ok(acc * lookup_value(lookup, idx)?))?;
            value += contribution;
        }
        Ok(value)
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

fn lookup_value(lookup: &[f64], idx: u32) -> Result<f64> {
    lookup.get(idx as usize).copied().ok_or_else(|| Error {
        status: Status::InvalidArgument,
        message: format!("lookup has no value for variable index {idx}"),
    })
}
