use std::{marker::PhantomData, ops::Index};

use num::NumCast;

use lunamodel_types::{Bias, BinaryAssignment, IntegerAssignment, RealAssignment, SpinAssignment};

// #[derive(Debug, Clone, PartialEq)]
// pub struct ColElement<T>(pub Vec<T>);
#[derive(Debug, Clone, PartialEq)]
pub struct ColElement<T>(pub Vec<f64>, PhantomData<T>);

#[derive(Debug, Clone, PartialEq)]
pub enum Column {
    Binary(ColElement<BinaryAssignment>),
    Spin(ColElement<SpinAssignment>),
    Integer(ColElement<IntegerAssignment>),
    Real(ColElement<RealAssignment>),
}

pub enum Assignment {
    Binary(BinaryAssignment),
    Spin(SpinAssignment),
    Integer(IntegerAssignment),
    Real(RealAssignment),
}

impl Index<usize> for Column {
    type Output = Bias;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            Self::Binary(col) => &col.0[index],
            Self::Spin(col) => &col.0[index],
            Self::Integer(col) => &col.0[index],
            Self::Real(col) => &col.0[index],
        }
    }
}

impl Column {
    pub fn as_assignment(&self, index: usize) -> Assignment {
        match self {
            Self::Binary(col) => Assignment::Binary(col.as_t(index)),
            Self::Spin(col) => Assignment::Spin(col.as_t(index)),
            Self::Integer(col) => Assignment::Integer(col.as_t(index)),
            Self::Real(col) => Assignment::Real(col.as_t(index)),
        }
    }
}

impl<T: NumCast> ColElement<T> {
    pub fn as_t(&self, index: usize) -> T {
        <T as NumCast>::from(self.0[index]).unwrap()
    }
}
