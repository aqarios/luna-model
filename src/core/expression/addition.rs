use super::{
    base::{ExpressionBaseAdd, ExpressionBaseCreation},
    Expression,
};
use crate::core::expression::One;
use crate::core::VarRef;
use crate::errors::VariablesFromDifferentEnvsErr;
use crate::{
    core::operations::{
        AddAssignToExpression, AddToExpression, SubAssignToExpression, SubToExpression,
    },
    types::Bias,
};

// ADDITION

impl AddToExpression<Bias> for &Expression {
    type Output = Expression;
    fn add(self, rhs: Bias) -> Self::Output {
        let mut out = Expression::new_from_other(&self);
        out.add_offset(rhs);
        out
    }
}

impl AddToExpression<&VarRef> for &Expression {
    type Output = Result<Expression, VariablesFromDifferentEnvsErr>;
    fn add(self, rhs: &VarRef) -> Self::Output {
        if self.env.id() != rhs.env.id() {
            Err(VariablesFromDifferentEnvsErr)
        } else {
            let mut out = Expression::new_from_other(&self);
            out.add_linear(rhs.id, Bias::one());
            Ok(out)
        }
    }
}

impl AddToExpression<&Expression> for &Expression {
    type Output = Result<Expression, VariablesFromDifferentEnvsErr>;
    fn add(self, rhs: &Expression) -> Self::Output {
        if self.env.id() != rhs.env.id() {
            Err(VariablesFromDifferentEnvsErr)
        } else {
            let mut out = Expression::new_from_other(&self);
            // We know that both expressions have the same environment
            // so we just need to check if the sizes of the two expression matches, i.e.
            // if both expressions have the same number of variables.
            // If rhs has more variables than self, we need to resize the out to
            // allow the other variables to be added safely.
            if out.active.len() < rhs.active.len() {
                out.active.resize(rhs.active.len(), false);
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
impl AddAssignToExpression<Bias> for Expression {
    type Output = ();

    fn add_assign(&mut self, rhs: Bias) -> Self::Output {
        self.add_offset(rhs)
    }
}

impl AddAssignToExpression<&VarRef> for Expression {
    type Output = Result<(), VariablesFromDifferentEnvsErr>;

    fn add_assign(&mut self, rhs: &VarRef) -> Self::Output {
        if self.env.id() != rhs.env.id() {
            Err(VariablesFromDifferentEnvsErr)
        } else {
            Ok(self.add_linear(rhs.id, Bias::one()))
        }
    }
}

impl AddAssignToExpression<&Expression> for Expression {
    type Output = Result<(), VariablesFromDifferentEnvsErr>;
    fn add_assign(&mut self, rhs: &Expression) -> Self::Output {
        if self.env.id() != rhs.env.id() {
            Err(VariablesFromDifferentEnvsErr)
        } else {
            // if self.active.len() < rhs.active.len() {
            //     self.resize(rhs.active.len().into());
            // }
            self.add_offset(rhs.offset);
            self.add_linear_from(&rhs.linear);

            if rhs.quadratic.is_some() {
                self.add_quadratic_from(rhs.quadratic.as_ref().unwrap());
            }
            if rhs.higher_order.is_some() {
                self.add_higher_order_from(rhs.higher_order.as_ref().unwrap());
            }
            Ok(())
        }
    }
}

// SUBTRACTION

impl SubToExpression<Bias> for &Expression {
    type Output = Expression;
    fn sub(self, rhs: Bias) -> Self::Output {
        self.add(-rhs)
    }
}

impl SubToExpression<&VarRef> for &Expression {
    type Output = Result<Expression, VariablesFromDifferentEnvsErr>;
    fn sub(self, rhs: &VarRef) -> Self::Output {
        if self.env.id() != rhs.env.id() {
            Err(VariablesFromDifferentEnvsErr)
        } else {
            let mut out = Expression::new_from_other(&self);
            out.add_linear(rhs.id, -Bias::one());
            Ok(out)
        }
    }
}

impl SubToExpression<&Expression> for &Expression {
    type Output = Result<Expression, VariablesFromDifferentEnvsErr>;
    fn sub(self, rhs: &Expression) -> Self::Output {
        if self.env.id() != rhs.env.id() {
            Err(VariablesFromDifferentEnvsErr)
        } else {
            let mut out = Expression::new_from_other(&self);
            // We know that both expressions have the same environment
            // so we just need to check if the sizes of the two expression matches, i.e.
            // if both expressions have the same number of variables.
            // If rhs has more variables than self, we need to resize the out to
            // allow the other variables to be added safely.
            if out.active.len() < rhs.active.len() {
                out.active.resize(rhs.active.len(), false);
            }
            // Now we can perform all additions safely.
            out.add_offset(-rhs.offset);
            out.add_linear_from(&(-&rhs.linear));

            if let Some(q) = &rhs.quadratic {
                out.add_quadratic_from(&(-q));
            }
            if let Some(ho) = &rhs.higher_order {
                out.add_higher_order_from(&(-ho));
            }
            Ok(out)
        }
    }
}

// SUBTRACTION ASSIGN

impl SubAssignToExpression<Bias> for Expression {
    type Output = ();

    fn sub_assign(&mut self, rhs: Bias) -> Self::Output {
        self.add_assign(-rhs)
    }
}

impl SubAssignToExpression<&VarRef> for Expression {
    type Output = Result<(), VariablesFromDifferentEnvsErr>;

    fn sub_assign(&mut self, rhs: &VarRef) -> Self::Output {
        if self.env.id() != rhs.env.id() {
            Err(VariablesFromDifferentEnvsErr)
        } else {
            Ok(self.add_linear(rhs.id, -Bias::one()))
        }
    }
}

impl SubAssignToExpression<&Expression> for Expression {
    type Output = Result<(), VariablesFromDifferentEnvsErr>;

    fn sub_assign(&mut self, rhs: &Expression) -> Self::Output {
        if self.env.id() != rhs.env.id() {
            Err(VariablesFromDifferentEnvsErr)
        } else {
            let result = self.add(&-rhs);
            match result {
                Ok(expr) => Ok(*self = expr),
                Err(e) => Err(e.into()),
            }
        }
    }
}
