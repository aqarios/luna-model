use std::{cell::RefCell, rc::Rc};

use super::{
    common::{MutRcConstraint, MutRcConstraints, MutRcExpression, MutRcModel, RcVarRef},
    term::{HigherOrder, Quadratic},
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

pub type ConcreteQuadratic = Quadratic<ConcreteIndex, ConcreteBias>;
pub type ConcreteHigherOrder = HigherOrder<ConcreteIndex, ConcreteBias>;

pub type ConcreteMutRcExpression = MutRcExpression<ConcreteIndex, ConcreteBias>;
pub type ConcreteMutRcEnvironment = MutRcEnvironment<ConcreteIndex>;
pub type ConcreteMutRcConstraint = MutRcConstraint<ConcreteIndex, ConcreteBias>;
pub type ConcreteMutRcConstraints = MutRcConstraints<ConcreteIndex, ConcreteBias>;
pub type ConcreteMutRcModel = MutRcModel<ConcreteIndex, ConcreteBias>;
pub type ConcreteRcVarRef = RcVarRef<ConcreteIndex>;

pub trait Create {
    fn create() -> Self;
}

impl Create for ConcreteMutRcConstraints {
    fn create() -> Self {
        Rc::new(RefCell::new(ConcreteConstraints::default()))
    }
}
