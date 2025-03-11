use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

use prost::{Enumeration, Message};

use crate::core::{Comparator, Constraint, Constraints, Environment, VarId};

use super::ser_expression::SerializableExpression;

#[derive(Debug, Clone, PartialEq, Enumeration)]
#[repr(i32)]
pub enum SerializableComparator {
    Eq = 0,
    Leq = 1,
    Geq = 2,
}

impl From<Comparator> for SerializableComparator {
    fn from(value: Comparator) -> Self {
        match value {
            Comparator::Leq => SerializableComparator::Leq,
            Comparator::Eq => SerializableComparator::Eq,
            Comparator::Geq => SerializableComparator::Geq,
        }
    }
}

impl Into<Comparator> for SerializableComparator {
    fn into(self) -> Comparator {
        match self {
            SerializableComparator::Leq => Comparator::Leq,
            SerializableComparator::Eq => Comparator::Eq,
            SerializableComparator::Geq => Comparator::Geq,
        }
    }
}

#[derive(Clone, PartialEq, Message)]
pub struct SerializableConstraint {
    #[prost(message, tag = "1")]
    lhs: Option<SerializableExpression>,
    #[prost(double, tag = "2")]
    rhs: f64,
    #[prost(enumeration = "SerializableComparator", tag = "3")]
    comparator: i32,
}

impl SerializableConstraint {
    fn new(
        lhs: Option<SerializableExpression>,
        rhs: f64,
        comparator: SerializableComparator,
    ) -> Self {
        Self {
            lhs,
            rhs,
            comparator: comparator.into(),
        }
    }
}

impl SerializableConstraint {
    fn extract(&self, environment: Rc<RefCell<Environment<VarId>>>) -> Constraint<VarId, f64> {
        let lhs = self.lhs.as_ref().unwrap().extract(environment);
        let rhs = self.rhs;
        let comparator = self.comparator().into();
        Constraint::new(Rc::new(RefCell::new(lhs)), rhs, comparator)
    }
}

#[derive(Clone, PartialEq, Message)]
pub struct SerializableConstraints {
    #[prost(message, repeated, tag = "1")]
    constraints: Vec<SerializableConstraint>,
}

impl SerializableConstraints {
    pub fn new(constraints: Ref<'_, Constraints<VarId, f64>>) -> Self {
        Self {
            constraints: Self::build_constraints(&constraints.constraints),
        }
    }

    pub fn build_constraints(
        constraints: &Vec<Constraint<VarId, f64>>,
    ) -> Vec<SerializableConstraint> {
        constraints
            .iter()
            .map(|constr| {
                SerializableConstraint::new(
                    Some(SerializableExpression::new(&constr.lhs.borrow())),
                    constr.rhs,
                    constr.comparator.into(),
                )
            })
            .collect()
    }

    pub fn extract(&self, environment: Rc<RefCell<Environment<VarId>>>) -> Constraints<VarId, f64> {
        Constraints::new_from_vec(
            self.constraints
                .iter()
                .map(|c| c.extract(Rc::clone(&environment)))
                .collect(),
        )
    }
}
