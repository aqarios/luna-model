use super::{
    base::{
        BiasConstraints, ExpressionBase, ExpressionBaseAdjustment, ExpressionBaseCreation,
        ExpressionBaseMul, ExpressionBaseMulDirect, IndexConstraints,
    },
    errors::VariableError,
    Expression,
};
use crate::core::exceptions::VariablesFromDifferentEnvsError;
use crate::core::operations::{MulAssignToExpression, MulToExpression};
use crate::core::VarRef;
use std::cell::Ref;

// MULTIPLICATION

impl<Index, Bias> MulToExpression<Index, Bias, Bias> for &Expression<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    type Output = Expression<Index, Bias>;

    fn mul(self, rhs: Bias) -> Self::Output {
        let mut out = Expression::new_from_other(&self);
        // Can do everything on out as it is equal to self.
        out.offset *= rhs;
        out.linear *= rhs;
        if out.has_quadratic() {
            *out.quadratic.as_mut().unwrap() *= rhs;
        }
        if out.has_higher_order() {
            *out.higher_order.as_mut().unwrap() *= rhs;
        }
        out
    }
}

impl<Index, Bias> MulToExpression<Index, Bias, &VarRef<Index>> for &Expression<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    type Output = Result<Expression<Index, Bias>, VariableError>;

    fn mul(self, rhs: &VarRef<Index>) -> Self::Output {
        if self.env.borrow().id != rhs.env.borrow().id {
            Err(VariablesFromDifferentEnvsError.into())
        } else {
            let mut out = Expression::new(self.env.clone());
            out.add_variables(self.num_variables().into());
            out.add_variables(rhs.id);

            out.mul_with_offset(self.offset, rhs.id, Bias::one())
                .map_err(|e| e.into())?;
            out.mul_with_linear(&self.linear, rhs.id, Bias::one())
                .map_err(|e| e.into())?;
            if self.has_quadratic() {
                // Don't need to do anything if the quadratic term is empty (is 0)
                out.enforce_quadratic();
                out.mul_with_quadratic(&self.quadratic.as_ref().unwrap(), rhs.id, Bias::one())
                    .map_err(|e| e.into())?;
            }
            if self.has_higher_order() {
                // Don't need to do anything if the higher order term is empty (is 0)
                out.enforce_higher_order();
                out.mul_with_higher_order(
                    &self.higher_order.as_ref().unwrap(),
                    rhs.id,
                    Bias::one(),
                )
                .map_err(|e| e.into())?;
            }
            Ok(out)
        }
    }
}

impl<Index, Bias> MulToExpression<Index, Bias, Ref<'_, Expression<Index, Bias>>>
    for &Expression<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    type Output = Result<Expression<Index, Bias>, VariablesFromDifferentEnvsError>;

    fn mul(self, rhs: Ref<'_, Expression<Index, Bias>>) -> Self::Output {
        if self.env.borrow().id != rhs.env.borrow().id {
            Err(VariablesFromDifferentEnvsError)
        } else {
            let mut out = Expression::new(self.env.clone());
            out.add_variables(self.num_variables().into());
            out.add_variables(rhs.num_variables().into());

            out.mul_offset(self.offset, rhs.offset);
            out.mul_linear(&self.linear, &rhs.linear);
            if self.has_quadratic() && rhs.has_quadratic() {
                // Only if both expressions have quadratic terms, we need to multiply
                // otherwise the result is always 0.
                out.mul_quadratic(
                    self.quadratic.as_ref().unwrap(),
                    rhs.quadratic.as_ref().unwrap(),
                );
            }
            if self.has_higher_order() && rhs.has_higher_order() {
                // Only if both expressions have higher order terms, we need to multiply
                // otherwise the result is always 0.
                out.mul_higher_order(
                    self.higher_order.as_ref().unwrap(),
                    rhs.higher_order.as_ref().unwrap(),
                );
            }
            Ok(out)
        }
    }
}

// MULTIPLICATION ASSIGN

impl<Index, Bias> MulAssignToExpression<Index, Bias, Bias> for Expression<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    type Output = ();

    fn mul_assign(&mut self, rhs: Bias) -> Self::Output {
        self.offset *= rhs;
        self.linear *= rhs;
        if self.has_quadratic() {
            *self.quadratic.as_mut().unwrap() *= rhs;
        }
        if self.has_higher_order() {
            *self.higher_order.as_mut().unwrap() *= rhs;
        }
    }
}

impl<Index, Bias> MulAssignToExpression<Index, Bias, &VarRef<Index>> for Expression<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    type Output = Result<(), VariableError>;

    fn mul_assign(&mut self, rhs: &VarRef<Index>) -> Self::Output {
        if self.env.borrow().id != rhs.env.borrow().id {
            Err(VariablesFromDifferentEnvsError.into())
        } else {
            // We use the `mul` implementation as we need the temporary expression.
            // We cannot simply just mutiply to itsel as some unforseeable changes
            // might be applied to the self expression. This needs to be checked
            // however, further performance improvements might be possible.
            let result = self.mul(rhs);
            match result {
                Ok(expr) => *self = expr,
                Err(e) => return Err(e.into()),
            };

            Ok(())
        }
    }
}

impl<Index, Bias> MulAssignToExpression<Index, Bias, Ref<'_, Expression<Index, Bias>>>
    for Expression<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    type Output = Result<(), VariablesFromDifferentEnvsError>;

    fn mul_assign(&mut self, rhs: Ref<'_, Expression<Index, Bias>>) -> Self::Output {
        if self.env.borrow().id != rhs.env.borrow().id {
            Err(VariablesFromDifferentEnvsError)
        } else {
            todo!();

            Ok(())
        }
    }
}
