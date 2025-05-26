use crate::core::expression::One;
use num::NumCast;
use std::fmt::{Debug, Display, LowerExp};

pub trait AssignmentBaseTypes: Debug + Clone + Copy + Default {
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
    + Clone
    + Display
    + ToString
    + Copy
    + Default
    + One
    + PartialEq
    + PartialOrd
    + NumCast
    + LowerExp
{
}

impl<
        T: Debug
            + Clone
            + Display
            + Copy
            + Default
            + One
            + PartialEq
            + PartialOrd
            + NumCast
            + LowerExp,
    > AssignmentConstraints for T
{
}
