use crate::core::expression::One;
use crate::core::traits::ContentEquality;
use crate::errors::VariableNotExistingErr;
use crate::{
    core::{
        environment::SharedEnvironment,
        expression::ExpressionBaseCreation,
        operations::{
            AddToExpression, MulToExpression, NegToExpression, RSubToExpression, SubToExpression,
        },
        Expression,
    },
    errors::VariablesFromDifferentEnvsErr,
    types::{Bias, VarIndex},
};
use std::fmt::{Debug, Display, Formatter};

/// A reference to a variable.
#[derive(Clone)]
pub struct VarRef {
    pub id: VarIndex,
    pub env: SharedEnvironment,
}

impl VarRef {
    pub fn new(id: VarIndex, env: SharedEnvironment) -> Self {
        Self { id, env }
    }

    pub fn check_living(&self) -> Result<(), VariableNotExistingErr> {
        self.env.access().check_living(self.id)
    }
}

impl AddToExpression<Bias> for &VarRef {
    type Output = Expression;

    fn add(self, rhs: Bias) -> Self::Output {
        Expression::new_linear_and_offset(self.env.clone(), self.id, Bias::one(), rhs)
    }
}

impl AddToExpression<&VarRef> for &VarRef {
    type Output = Result<Expression, VariablesFromDifferentEnvsErr>;

    fn add(self, rhs: &VarRef) -> Self::Output {
        if self.env.id() != rhs.env.id() {
            Err(VariablesFromDifferentEnvsErr)
        } else {
            Ok(Expression::new_linear(
                self.env.clone(),
                (self.id, Bias::one()),
                (rhs.id, Bias::one()),
            ))
        }
    }
}

impl MulToExpression<Bias> for &VarRef {
    type Output = Expression;

    fn mul(self, rhs: Bias) -> Self::Output {
        Expression::new_linear_single(self.env.clone(), self.id, rhs)
    }
}

impl MulToExpression<&VarRef> for &VarRef {
    type Output = Result<Expression, VariablesFromDifferentEnvsErr>;

    fn mul(self, rhs: &VarRef) -> Self::Output {
        if self.env.id() != rhs.env.id() {
            Err(VariablesFromDifferentEnvsErr)
        } else {
            Ok(Expression::new_quadratic(
                self.env.clone(),
                self.id,
                rhs.id,
                Bias::one(),
            ))
        }
    }
}

impl RSubToExpression<Bias> for &VarRef {
    type Output = Expression;

    fn rsub(self, rhs: Bias) -> Self::Output {
        Expression::new_linear_and_offset(self.env.clone(), self.id, -Bias::one(), rhs)
    }
}

impl SubToExpression<&VarRef> for &VarRef {
    type Output = Result<Expression, VariablesFromDifferentEnvsErr>;

    fn sub(self, rhs: &VarRef) -> Self::Output {
        if self.env.id() != rhs.env.id() {
            Err(VariablesFromDifferentEnvsErr)
        } else {
            Ok(Expression::new_linear(
                self.env.clone(),
                (self.id, Bias::one()),
                (rhs.id, -Bias::one()),
            ))
        }
    }
}

impl Debug for VarRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let idx: usize = self.id.into();
        let v = &self.env.access()[idx];
        write!(f, "{v:?}")
    }
}

impl Display for VarRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let idx: usize = self.id.into();
        let v = &self.env.access()[idx];
        f.write_str(&v.to_string())
    }
}

impl NegToExpression for &VarRef {
    type Output = Expression;

    fn neg(self) -> Self::Output {
        Expression::new_linear_single(self.env.clone(), self.id, -Bias::one())
    }
}

impl ContentEquality for VarRef {
    fn is_equal_contents(&self, other: &Self) -> bool {
        self.id == other.id && self.env.is_equal_contents(&other.env)
    }
}
