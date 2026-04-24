use std::ops::Mul;

use itertools::Itertools;
use lunamodel_error::LunaModelResult;

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

        let mut result = Expression::empty(self.env.clone());
        for (vars, bias) in self.items() {
            match &vars[..] {
                [] => result.add_assign(bias)?,
                [u] => match u.id == target.id {
                    true => result.add_assign(replacement.mul(bias)?)?,
                    false => result.linear += (u.id, bias),
                },
                [u, v] => match (u.id == target.id, v.id == target.id) {
                    (true, true) => result.add_assign(replacement.pow(2)?.mul(bias)?)?,
                    (true, false) => result.add_assign(replacement.mul(v)?.mul(bias)?)?,
                    (false, true) => result.add_assign(replacement.mul(u)?.mul(bias)?)?,
                    (false, false) => {
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
                    let vidxs = vs.iter().map(|v| v.id).collect_vec();
                    match vidxs.contains(&target.id) {
                        true => {
                            let newho = vs.iter().fold(
                                Expression::constant(self.env.clone(), bias),
                                |e, v| match v.id == target.id {
                                    true => e.mul(replacement).unwrap(),
                                    false => e.mul(v).unwrap(),
                                },
                            );
                            result.add_assign(newho)?;
                        }
                        false => {
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
        }
        Ok(result)
    }
}
