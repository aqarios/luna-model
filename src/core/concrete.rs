use std::{cell::RefCell, rc::Rc};

use super::{
    expression::{BiasConstraints, IndexConstraints},
    types::{MutRcConstraint, MutRcConstraints},
    Constraint, Constraints, Environment, Expression, Model, MutRcEnvironment, VarId, VarRef,
};

pub type ConcreteId = u32;
pub type ConcreteIndex = VarId;
pub type ConcreteBias = f64;
pub type ConcreteEnvId = u8;

pub type ConcreteVarRef = VarRef<ConcreteIndex>;
pub type ConcreteExpression = Expression<ConcreteIndex, ConcreteBias>;
pub type ConcreteConstraint = Constraint<ConcreteIndex, ConcreteBias>;
pub type ConcreteConstraints = Constraints<ConcreteIndex, ConcreteBias>;
pub type ConcreteModel = Model<ConcreteIndex, ConcreteBias>;
pub type ConcreteEnvironment = Environment<ConcreteIndex>;

pub type ConcreteMutRcEnvironment = MutRcEnvironment<ConcreteIndex>;
pub type ConcreteMutRcConstraint = MutRcConstraint<ConcreteIndex, ConcreteBias>;
pub type ConcreteMutRcConstraints = MutRcConstraints<ConcreteIndex, ConcreteBias>;
pub type ConcreteMutRcModel = Rc<RefCell<ConcreteModel>>;
pub type ConcreteRcVarRef = Rc<VarRef<VarId>>;

pub trait Create {
    fn create() -> Self;
}

impl Create for ConcreteMutRcConstraints {
    fn create() -> Self {
        Rc::new(RefCell::new(ConcreteConstraints::default()))
    }
}

impl Into<MutRcConstraints> for ConcreteConstraints {
    fn into(self) -> MutRcConstraints {
        RefCell::new(self).into()
    }
}

impl Into<MutRcEnvironment> for ConcreteEnvironment {
    fn into(self) -> MutRcEnvironment {
        RefCell::new(self).into()
    }
}

impl<Index: IndexConstraints, Bias: BiasConstraints> Into<MutRcExpression<Index, Bias>>
    for Expression<Index, Bias>
{
    fn into(self) -> MutRcExpression<Index, Bias> {
        Rc::new(RefCell::new(self))
    }
}
