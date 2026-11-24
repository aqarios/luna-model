//! Substitution helpers for expressions.

use std::ops::Mul;

use itertools::Itertools;
use lunamodel_error::LunaModelResult;
use lunamodel_types::VarIdx;

use crate::{
    ops::{LmAddAssign, LmPow, utils::check_envs},
    prelude::{Expression, HigherOrder, Quadratic, VarRef},
};

impl Expression {
    /// Replaces every occurrence of `target` with `replacement`.
    ///
    /// The substitution is performed term-by-term and preserves the current
    /// environment. Any resulting higher-degree terms are rebuilt through the
    /// normal expression arithmetic APIs.
    pub fn substitute(
        &self,
        target: &VarRef,
        replacement: &Expression,
    ) -> LunaModelResult<Expression> {
        check_envs(self, target)?;
        check_envs(self, replacement)?;
        target.check_living()?;
        self.substitute_with(|id| (id == target.id).then_some(replacement))
    }

    /// Replaces every variable for which `subs` returns a replacement, in one
    /// walk over the expression. The substitution is simultaneous: variables
    /// inside a replacement are not substituted again. Callers validate
    /// environments and liveness first.
    pub(crate) fn substitute_with<'r>(
        &self,
        subs: impl Fn(VarIdx) -> Option<&'r Expression>,
    ) -> LunaModelResult<Expression> {
        let mut result = Expression::empty(self.env.clone());
        for (vars, bias) in self.items() {
            match &vars[..] {
                [] => result.add_assign(bias)?,
                [u] => match subs(u.id) {
                    Some(r) => result.add_assign(r.mul(bias)?)?,
                    None => result.linear += (u.id, bias),
                },
                [u, v] => match (subs(u.id), subs(v.id)) {
                    (Some(r), Some(_)) if u.id == v.id => {
                        result.add_assign(r.pow(2)?.mul(bias)?)?
                    }
                    (Some(ru), Some(rv)) => result.add_assign(ru.mul(rv)?.mul(bias)?)?,
                    (Some(r), None) => result.add_assign(r.mul(v)?.mul(bias)?)?,
                    (None, Some(r)) => result.add_assign(r.mul(u)?.mul(bias)?)?,
                    (None, None) => {
                        if let Some(q) = result.quadratic.as_mut() {
                            *q += (u.id, v.id, bias);
                        } else {
                            let mut q = Quadratic::default();
                            q += (u.id, v.id, bias);
                            result.quadratic = Some(q);
                        }
                    }
                },
                vs => {
                    if vs.iter().any(|v| subs(v.id).is_some()) {
                        let newho = vs.iter().try_fold(
                            Expression::constant(self.env.clone(), bias),
                            |e, v| match subs(v.id) {
                                Some(r) => e.mul(r),
                                None => e.mul(v),
                            },
                        )?;
                        result.add_assign(newho)?;
                    } else {
                        let vidxs = vs.iter().map(|v| v.id).collect_vec();
                        if let Some(h) = result.higher_order.as_mut() {
                            *h += (vidxs.as_slice(), bias);
                        } else {
                            let mut h = HigherOrder::default();
                            h += (vidxs.as_slice(), bias);
                            result.higher_order = Some(h);
                        }
                    }
                }
            }
        }
        Ok(result)
    }
}
