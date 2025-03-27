use crate::{
    core::{
        expression::{BiasConstraints, ExpressionBaseCreation, IndexConstraints},
        operations::{
            AddToExpression, MulToExpression, NegToExpression, RSubToExpression, SubToExpression,
        },
        Expression, MutRcEnvironment,
    },
    errors::VariablesFromDifferentEnvsError,
};
use std::{
    fmt::{Debug, Display, Formatter},
    rc::Rc,
};

/// A reference to a variable.
#[derive(Clone)]
pub struct VarRef<Index> {
    pub id: Index,
    pub env: MutRcEnvironment<Index>,
}

impl<Index> VarRef<Index>
where
    Index: IndexConstraints,
{
    pub fn new(id: Index, env: MutRcEnvironment<Index>) -> Self {
        Self { id, env }
    }
}

impl<Index, Bias> AddToExpression<Index, Bias, Bias> for &VarRef<Index>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    type Output = Expression<Index, Bias>;

    fn add(self, rhs: Bias) -> Self::Output {
        Expression::new_linear_and_offset(Rc::clone(&self.env), self.id, Bias::one(), rhs)
    }
}

impl<Index, Bias> AddToExpression<Index, Bias, &VarRef<Index>> for &VarRef<Index>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    type Output = Result<Expression<Index, Bias>, VariablesFromDifferentEnvsError>;

    fn add(self, rhs: &VarRef<Index>) -> Self::Output {
        if self.env.borrow().id != rhs.env.borrow().id {
            Err(VariablesFromDifferentEnvsError)
        } else {
            Ok(Expression::new_linear(
                Rc::clone(&self.env),
                (self.id, Bias::one()),
                (rhs.id, Bias::one()),
            ))
        }
    }
}

impl<Index, Bias> MulToExpression<Index, Bias, Bias> for &VarRef<Index>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    type Output = Expression<Index, Bias>;

    fn mul(self, rhs: Bias) -> Self::Output {
        Expression::new_linear_single(Rc::clone(&self.env), self.id, rhs)
    }
}

impl<Index, Bias> MulToExpression<Index, Bias, &VarRef<Index>> for &VarRef<Index>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    type Output = Result<Expression<Index, Bias>, VariablesFromDifferentEnvsError>;

    fn mul(self, rhs: &VarRef<Index>) -> Self::Output {
        if self.env.borrow().id != rhs.env.borrow().id {
            Err(VariablesFromDifferentEnvsError)
        } else {
            Ok(Expression::new_quadratic(
                Rc::clone(&self.env),
                self.id,
                rhs.id,
                Bias::one(),
            ))
        }
    }
}

impl<Index, Bias> RSubToExpression<Index, Bias, Bias> for &VarRef<Index>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    type Output = Expression<Index, Bias>;

    fn rsub(self, rhs: Bias) -> Self::Output {
        Expression::new_linear_and_offset(Rc::clone(&self.env), self.id, -Bias::one(), rhs)
    }
}

impl<Index, Bias> SubToExpression<Index, Bias, &VarRef<Index>> for &VarRef<Index>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    type Output = Result<Expression<Index, Bias>, VariablesFromDifferentEnvsError>;

    fn sub(self, rhs: &VarRef<Index>) -> Self::Output {
        if self.env.borrow().id != rhs.env.borrow().id {
            Err(VariablesFromDifferentEnvsError)
        } else {
            Ok(Expression::new_linear(
                Rc::clone(&self.env),
                (self.id, Bias::one()),
                (rhs.id, -Bias::one()),
            ))
        }
    }
}

impl<Index> Debug for VarRef<Index>
where
    Index: IndexConstraints,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let env = self.env.borrow();
        let v = &env.variables[self.id.into()];

        write!(f, "{v:?}")
    }
}

impl<Index> Display for VarRef<Index>
where
    Index: IndexConstraints,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let v = &self.env.borrow().variables[self.id.into()];
        f.write_str(&v.to_string())
    }
}

impl<Index, Bias> NegToExpression<Index, Bias> for &VarRef<Index>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    type Output = Expression<Index, Bias>;

    fn neg(self) -> Self::Output {
        Expression::new_linear_single(Rc::clone(&self.env), self.id, -Bias::one())
    }
}
