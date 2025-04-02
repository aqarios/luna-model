use num::NumCast;
use std::fmt::{Debug, Display};

pub trait AssignmentBaseTypes: Debug + Clone + Copy {
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
    Debug + Clone + Display + Copy + Default + PartialEq + PartialOrd + NumCast
{
}

impl<T: Debug + Clone + Display + Copy + Default + PartialEq + PartialOrd + NumCast>
    AssignmentConstraints for T
{
}
