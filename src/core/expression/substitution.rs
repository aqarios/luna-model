use crate::{
    core::{
        expression::{ExpressionBaseAdd, ExpressionBaseCreation},
        operations::{AddAssignToExpression, MulAssignToExpression, MulToExpression},
        VarRef,
    },
    errors::{DifferentEnvsErr, VariablesFromDifferentEnvsErr}, types::{Bias, VarIndex},
};

use super::{Expression, ExpressionBaseAdjustment};

pub trait Substitution {
    /// Substitute every occurrence of a variable in an expression with another expression.
    ///
    /// Given an expression `self`, this method replaces all occurrences
    /// of `target` with `replacement`. If the substitution would cross differing
    /// environments (e.g. captures from two different scopes), it returns a `DifferentEnvsErr`.
    ///
    /// # Parameters
    /// - `target`: the variable reference to replace
    /// - `replacement`: the expression to insert in place of `target`
    ///
    /// # Returns
    /// - `Ok(Expression)`: the resulting expression after substitution
    /// - `Err(DifferentEnvsErr)`: if the environments of `self`, `target` and `replacement`
    ///    are not compatible
    fn substitute(
        self,
        target: &VarRef,
        replacement: &Expression,
    ) -> Result<Expression, DifferentEnvsErr>;
}

impl Substitution for &Expression {
    fn substitute(
        self,
        target: &VarRef,
        replacement: &Expression,
    ) -> Result<Expression, DifferentEnvsErr> {
        let env_self_and_var_match = self.env.id() == target.env.id();
        let env_self_and_target_match = self.env.id() == replacement.env.id();
        if !env_self_and_var_match || !env_self_and_target_match {
            return Err(DifferentEnvsErr);
        }

        let mut out = Expression::empty(self.env.clone());
        out.offset += self.offset;

        let active_linears: Vec<(VarIndex, Bias)> = self
            .linear
            .iter()
            .filter(|(idx, _)| self.active[*idx])
            .map(|(idx, bias)| (idx.into(), *bias))
            .collect();

        for (var, bias) in active_linears.iter() {
            if *var != target.id {
                out.add_linear(*var, *bias);
            } else {
                out.add_assign(&replacement.mul(*bias))?;
            }
        }

        if let Some(quad) = &self.quadratic {
            for (u, v, bias) in quad.iter_flat() {
                match (u == target.id, v == target.id) {
                    (true, true) => {
                        let mut toadd = replacement.mul(replacement)?;
                        toadd.mul_assign(bias);
                        out.add_assign(&toadd)?
                    }
                    (true, false) => {
                        out.add_assign(&replacement.mul(&VarRef::new(v, self.env.clone()))?)?
                    }
                    (false, true) => {
                        out.add_assign(&replacement.mul(&VarRef::new(u, self.env.clone()))?)?
                    }
                    (false, false) => out.add_quadratic(u, v, bias),
                }
            }
        }

        if let Some(ho) = &self.higher_order {
            for (indices, bias) in ho.iter_contrib() {
                if indices.contains(&target.id) {
                    let mut toadd = Expression::simple(self.env.clone(), *bias);
                    for var in indices.iter() {
                        if *var == target.id {
                            toadd.mul_assign(replacement)?;
                        } else {
                            toadd.mul_assign(&VarRef::new(*var, self.env.clone()))?;
                        }
                    }
                    out.add_assign(&toadd)?;
                } else {
                    out.add_higher_order(&indices, *bias);
                }
            }
        }

        out.remove_variable(target.id);
        Ok(out)
    }
}

impl Substitution for Expression {
    fn substitute(self, var: &VarRef, target: &Expression) -> Result<Expression, DifferentEnvsErr> {
        (&self).substitute(var, target)
    }
}

impl From<VariablesFromDifferentEnvsErr> for DifferentEnvsErr {
    fn from(_: VariablesFromDifferentEnvsErr) -> Self {
        Self {}
    }
}
