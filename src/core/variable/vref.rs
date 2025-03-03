use crate::core::{
    exceptions::{ParseFromStringError, VariablesFromDifferentEnvsError},
    expression::{BiasConstraints, ExpressionBaseCreation, IndexConstraints, One},
    operations::{AddToExpression, MulToExpression},
    Environment, Expression,
};
use std::fmt::{Debug, Display, Formatter};
use std::{cell::RefCell, ops::AddAssign, rc::Rc, str::FromStr};

#[derive(Debug, Clone, Copy, Default, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct VarId(pub u32);

impl One for VarId {
    fn one() -> Self {
        VarId(1)
    }
}

impl AddAssign<VarId> for VarId {
    fn add_assign(&mut self, rhs: VarId) {
        self.0 += rhs.0
    }
}

impl ToString for VarId {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl FromStr for VarId {
    type Err = ParseFromStringError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<u32>()
            .map(VarId)
            .map_err(|e| ParseFromStringError(e.to_string()))
    }
}

impl Into<usize> for VarId {
    fn into(self) -> usize {
        self.0 as usize
    }
}

impl From<usize> for VarId {
    fn from(value: usize) -> Self {
        assert!(value <= u32::MAX as usize, "value out of range for u32");
        VarId(value as u32)
    }
}

impl Into<u64> for VarId {
    fn into(self) -> u64 {
        self.0 as u64
    }
}

#[derive(Clone)]
pub struct VarRef<Index> {
    pub id: Index,
    pub env: Rc<RefCell<Environment<Index>>>,
}

impl<Index> VarRef<Index>
where
    Index: IndexConstraints,
{
    pub fn new(id: Index, env: Rc<RefCell<Environment<Index>>>) -> Self {
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
        Expression::new_linear_single(self.env.clone(), self.id, rhs)
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
                self.env.clone(),
                self.id,
                rhs.id,
                Bias::one(),
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
        Expression::new_linear_single(self.env.clone(), self.id, rhs)
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
                self.env.clone(),
                self.id,
                rhs.id,
                Bias::one(),
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
        write!(f, "{v}")
    }
}
