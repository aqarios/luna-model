use std::{
    fmt::{Display, Formatter},
    ops::{Mul, Sub},
};

use num::NumCast;

use crate::types::{
    Bias, BinaryAssignmentType, IntegerAssignmentType, RealAssignmentType, SpinAssignmentType,
};

#[derive(Debug, Clone, Copy)]
pub enum VarAssignment {
    Binary(BinaryAssignmentType),
    Spin(SpinAssignmentType),
    Integer(IntegerAssignmentType),
    Real(RealAssignmentType),
}

impl VarAssignment {
    pub fn to_bias(&self) -> Bias {
        match self {
            VarAssignment::Binary(col) => <Bias as NumCast>::from(*col).unwrap(),
            VarAssignment::Spin(col) => <Bias as NumCast>::from(*col).unwrap(),
            VarAssignment::Integer(col) => <Bias as NumCast>::from(*col).unwrap(),
            VarAssignment::Real(col) => <Bias as NumCast>::from(*col).unwrap(),
        }
    }
}

impl Default for VarAssignment {
    fn default() -> Self {
        VarAssignment::Binary(BinaryAssignmentType::default())
    }
}

impl Display for VarAssignment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            VarAssignment::Binary(x) => write!(f, "{x}"),
            VarAssignment::Spin(x) => write!(f, "{x}"),
            VarAssignment::Integer(x) => write!(f, "{x}"),
            VarAssignment::Real(x) => write!(f, "{x:?}"),
        }
    }
}

impl Mul<Bias> for VarAssignment {
    type Output = Bias;

    fn mul(self, rhs: Bias) -> Self::Output {
        match self {
            VarAssignment::Binary(col) => <Bias as NumCast>::from(col).unwrap() * rhs,
            VarAssignment::Spin(col) => <Bias as NumCast>::from(col).unwrap() * rhs,
            VarAssignment::Integer(col) => <Bias as NumCast>::from(col).unwrap() * rhs,
            VarAssignment::Real(col) => <Bias as NumCast>::from(col).unwrap() * rhs,
        }
    }
}

impl Sub<VarAssignment> for Bias {
    type Output = Bias;

    fn sub(self, rhs: VarAssignment) -> Self::Output {
        match rhs {
            VarAssignment::Binary(col) => self - <Bias as NumCast>::from(col).unwrap(),
            VarAssignment::Spin(col) => self - <Bias as NumCast>::from(col).unwrap(),
            VarAssignment::Integer(col) => self - <Bias as NumCast>::from(col).unwrap(),
            VarAssignment::Real(col) => self - <Bias as NumCast>::from(col).unwrap(),
        }
    }
}
