use crate::core::ConcreteBias;
use std::fmt::{Debug, Display};
use std::ops::Mul;

pub trait AssignmentBaseTypes {
    /// The type of binary variable assignments in solutions
    type BinaryType: AssignmentConstraints;
    /// The type of spin variable assignments in solutions
    type SpinType: AssignmentConstraints;
    /// The type of integer variable assignments in solutions
    type IntegerType: AssignmentConstraints;
    /// The type of real-valued variable assignments in solutions
    type RealType: AssignmentConstraints;
}

pub trait AssignmentConstraints:
    Debug
    + Display
    + Copy
    + Default
    + PartialEq
    + PartialOrd
    + Mul<ConcreteBias, Output = ConcreteBias>
    + Into<ConcreteBias>
{
}

impl<
        T: Debug
            + Display
            + Copy
            + Default
            + PartialEq
            + PartialOrd
            + Mul<ConcreteBias, Output = ConcreteBias>
            + Into<ConcreteBias>,
    > AssignmentConstraints for T
{
}
