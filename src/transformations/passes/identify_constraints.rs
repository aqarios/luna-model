// use crate::core::{Constraint, Comparator};

pub enum ConstraintTypes {
    GeneralEquality,
    GeneralInequality,
    IntegerEquality,
    IntegerInequality,
    SetPartitioning,
    SetPacking,
    SetCovering,
    HammingWeight,
}

impl ConstraintTypes {
    pub fn parent(&self) -> Option<ConstraintTypes> {
        match self {
            ConstraintTypes::SetPartitioning => Some(ConstraintTypes::HammingWeight),
            ConstraintTypes::HammingWeight => Some(ConstraintTypes::IntegerEquality),
            ConstraintTypes::SetPacking => Some(ConstraintTypes::IntegerInequality),
            ConstraintTypes::SetCovering => Some(ConstraintTypes::IntegerInequality),
            ConstraintTypes::IntegerEquality => Some(ConstraintTypes::GeneralEquality),
            ConstraintTypes::IntegerInequality => Some(ConstraintTypes::GeneralInequality),
            _ => None,
        }
    }

    // pub fn identify(constraint: &Constraint) -> ConstraintTypes {
    //     let is_eq = constraint.comparator == Comparator::Eq;
    //
    // }
}
