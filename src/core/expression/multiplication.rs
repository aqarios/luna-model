use super::{
    base::{
        BiasConstraints, ExpressionBaseCreation, ExpressionBaseMul, ExpressionBaseMulDirect,
        IndexConstraints,
    },
    Expression,
};
use crate::core::operations::{MulAssignToExpression, MulToExpression};
use crate::core::VarRef;
use crate::core::{exceptions::VariablesFromDifferentEnvsError, ExpressionBase};

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
    type Output = Result<Expression<Index, Bias>, VariablesFromDifferentEnvsError>;

    fn mul(self, rhs: &VarRef<Index>) -> Self::Output {
        if self.env.borrow().id != rhs.env.borrow().id {
            Err(VariablesFromDifferentEnvsError)
        } else {
            let mut out =
                Expression::new(self.env.clone(), self.active.clone(), self.num_variables());
            out.active = self.active.clone();
            out.num_variables = self.num_variables;
            out.mul_with_offset(self.offset, rhs.id, Bias::one());
            out.mul_with_linear(&self.linear, rhs.id, Bias::one());
            if self.has_quadratic() {
                // Don't need to do anything if the quadratic term is empty (is 0)
                out.enforce_quadratic();
                out.mul_with_quadratic(&self.quadratic.as_ref().unwrap(), rhs.id, Bias::one());
            }
            if self.has_higher_order() {
                // Don't need to do anything if the higher order term is empty (is 0)
                out.enforce_higher_order();
                out.mul_with_higher_order(
                    &self.higher_order.as_ref().unwrap(),
                    rhs.id,
                    Bias::one(),
                );
            }
            Ok(out)
        }
    }
}

impl<Index, Bias> MulToExpression<Index, Bias, &Expression<Index, Bias>>
    for &Expression<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    type Output = Result<Expression<Index, Bias>, VariablesFromDifferentEnvsError>;

    fn mul(self, rhs: &Expression<Index, Bias>) -> Self::Output {
        if self.env.borrow().id != rhs.env.borrow().id {
            Err(VariablesFromDifferentEnvsError)
        } else {
            let mut result = Expression::empty(self.env.clone());
            Expression::multiply(&self, &rhs, &mut result);
            Ok(result)
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
    type Output = Result<(), VariablesFromDifferentEnvsError>;

    fn mul_assign(&mut self, rhs: &VarRef<Index>) -> Self::Output {
        if self.env.borrow().id != rhs.env.borrow().id {
            Err(VariablesFromDifferentEnvsError)
        } else {
            // We use the `mul` implementation as we need the temporary expression.
            // We cannot simply just mutiply to itsel as some unforseeable changes
            // might be applied to the self expression. This needs to be checked
            // however, further performance improvements might be possible.
            let result = self.mul(rhs);
            match result {
                Ok(expr) => Ok(*self = expr),
                Err(e) => Err(e.into()),
            }
        }
    }
}

impl<Index, Bias> MulAssignToExpression<Index, Bias, &Expression<Index, Bias>>
    for Expression<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    type Output = Result<(), VariablesFromDifferentEnvsError>;

    fn mul_assign(&mut self, rhs: &Expression<Index, Bias>) -> Self::Output {
        if self.env.borrow().id != rhs.env.borrow().id {
            Err(VariablesFromDifferentEnvsError)
        } else {
            let result = self.mul(rhs);
            match result {
                Ok(expr) => Ok(*self = expr),
                Err(e) => Err(e.into()),
            }
        }
    }
}
