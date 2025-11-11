use crate::core::expression::One;
use crate::core::traits::ContentEquality;
use crate::core::{Environment, Vtype};
use crate::errors::{UnsupportedNotOperationErr, UnsupportedOperationErr, VariableNotExistingErr};
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
use std::ops::Not;

/// A reference to a variable.
#[derive(Clone, PartialEq)]
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

impl Not for &VarRef {
    type Output = Result<VarRef, UnsupportedOperationErr>;

    fn not(self) -> Self::Output {
        // Not is only implemented on binary variables.
        // Not on other vtypes results in an error.
        // let env = self.env.access_mut();
        let vtype = self.env.access().get_vtype(self.id);
        match vtype {
            Vtype::Binary => {
                // First, we need to check that this variable is not already inverted.
                let var = self.env.access()[self.id].clone();
                if let Some(inverted) = var.inverted {
                    // The variable was already inverted, so we can directly return it's
                    // inverted counterpart.
                    Ok(VarRef::new(inverted, self.env.clone()))
                } else {
                    // The variable does **not** have an inverted counter part.
                    // We need to create a new one and store this reference as an additional
                    // variable.
                    // todo: Do we really need to mention it in the variables lookup?
                    let inverted_id =
                        Environment::add_inverted_variable(&mut self.env.access_mut(), &var)
                            .unwrap();
                    // The inverted vars' inverted counterpart is the normal var.
                    self.env.access_mut()[inverted_id].inverted = Some(self.id);
                    self.env.access_mut()[self.id].inverted = Some(inverted_id);
                    Ok(VarRef::new(inverted_id, self.env.clone()))
                }
            }
            Vtype::InvertedBinary => {
                // An inverted variable is inverted again, so now, it's just the base variable.
                // At some point the base variable was already inverted. Otherwise we wouldn't have
                // the inverted variable so now, we can safely just return the base variable.
                let base_id = self.env.access()[self.id].inverted.unwrap();
                Ok(VarRef::new(base_id, self.env.clone()))
            }
            _ => Err(UnsupportedNotOperationErr::new(vtype)),
        }
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
