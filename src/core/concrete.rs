use super::{
    common::{MutRcConstraint, MutRcConstraints, MutRcExpression, MutRcModel, RcVarRef},
    term::{HigherOrder, Quadratic},
    Constraint, Constraints, Environment, Expression, Model, MutRcEnvironment, RcSolution, VarId,
    VarRef,
};
use crate::core::solution::AssignmentBaseTypes;
use std::{cell::RefCell, rc::Rc};

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

pub type ConcreteSolution = RcSolution<ConcreteBias, ConcreteAssignmentTypes>;

pub type ConcreteBinaryType = u8;
pub type ConcreteSpinType = i8;
pub type ConcreteIntegerType = i64;
pub type ConcreteRealType = f64;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct ConcreteAssignmentTypes {}

impl AssignmentBaseTypes for ConcreteAssignmentTypes {
    type BinaryType = ConcreteBinaryType;
    type SpinType = ConcreteSpinType;
    type IntegerType = ConcreteIntegerType;
    type RealType = ConcreteRealType;
}

pub trait Create {
    fn create() -> Self;
}

impl Create for ConcreteMutRcConstraints {
    fn create() -> Self {
        Rc::new(RefCell::new(ConcreteConstraints::default()))
    }
}
