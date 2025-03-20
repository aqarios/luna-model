use super::{
    base::{
        BiasConstraints, ExpressionBase, ExpressionBaseAdd, ExpressionBaseAdjustment,
        ExpressionBaseCreation, IndexConstraints,
    },
    Expression,
};
use crate::core::operations::{AddAssignToExpression, AddToExpression};
use crate::core::VarRef;
use crate::errors::VariablesFromDifferentEnvsError;

// ADDITION

impl<Index, Bias> AddToExpression<Index, Bias, Bias> for &Expression<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    type Output = Expression<Index, Bias>;
    fn add(self, rhs: Bias) -> Self::Output {
        let mut out = Expression::new_from_other(&self);
        out.add_offset(rhs);
        out
    }
}

impl<Index, Bias> AddToExpression<Index, Bias, &VarRef<Index>> for &Expression<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    type Output = Result<Expression<Index, Bias>, VariablesFromDifferentEnvsError>;
    fn add(self, rhs: &VarRef<Index>) -> Self::Output {
        if self.env.borrow().id != rhs.env.borrow().id {
            Err(VariablesFromDifferentEnvsError)
        } else {
            let mut out = Expression::new_from_other(&self);
            out.add_linear(rhs.id, Bias::one());
            Ok(out)
        }
    }
}

impl<Index, Bias> AddToExpression<Index, Bias, &Expression<Index, Bias>>
    for &Expression<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    type Output = Result<Expression<Index, Bias>, VariablesFromDifferentEnvsError>;
    fn add(self, rhs: &Expression<Index, Bias>) -> Self::Output {
        if self.env.borrow().id != rhs.env.borrow().id {
            Err(VariablesFromDifferentEnvsError)
        } else {
            let mut out = Expression::new_from_other(&self);
            // We know that both expressions have the same environment
            // so we just need to check if the sizes of the two expression matches, i.e.
            // if both expressions have the same number of variables.
            // If rhs has more variables than self, we need to resize the out to
            // allow the other variables to be added safely.
            if out.num_variables() < rhs.num_variables() {
                out.resize(rhs.num_variables().into());
            }
            // Now we can perform all additions safely.
            out.add_offset(rhs.offset);
            out.add_linear_from(&rhs.linear);

            if rhs.quadratic.is_some() {
                out.add_quadratic_from(rhs.quadratic.as_ref().unwrap());
            }
            if rhs.higher_order.is_some() {
                out.add_higher_order_from(rhs.higher_order.as_ref().unwrap());
            }
            Ok(out)
        }
    }
}

// ADD ASSIGN
impl<Index, Bias> AddAssignToExpression<Index, Bias, Bias> for Expression<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    type Output = ();

    fn add_assign(&mut self, rhs: Bias) -> Self::Output {
        self.add_offset(rhs)
    }
}

impl<Index, Bias> AddAssignToExpression<Index, Bias, &VarRef<Index>> for Expression<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    type Output = Result<(), VariablesFromDifferentEnvsError>;

    fn add_assign(&mut self, rhs: &VarRef<Index>) -> Self::Output {
        if self.env.borrow().id != rhs.env.borrow().id {
            Err(VariablesFromDifferentEnvsError)
        } else {
            Ok(self.add_linear(rhs.id, Bias::one()))
        }
    }
}

impl<Index, Bias> AddAssignToExpression<Index, Bias, &Expression<Index, Bias>>
    for Expression<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    type Output = Result<(), VariablesFromDifferentEnvsError>;
    fn add_assign(&mut self, rhs: &Expression<Index, Bias>) -> Self::Output {
        if self.env.borrow().id != rhs.env.borrow().id {
            Err(VariablesFromDifferentEnvsError)
        } else {
            let result = self.add(rhs);
            match result {
                Ok(expr) => Ok(*self = expr),
                Err(e) => Err(e.into()),
            }
        }
    }
}
