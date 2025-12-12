use std::{marker::PhantomData, ops::Index};

use lunamodel_error::{LunaModelError, LunaModelResult};
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

    pub fn push(&mut self, value: Assignment) -> LunaModelResult<()> {
        match self {
            Self::Binary(col) => col.push(value),
            Self::Spin(col) => col.push(value),
            Self::Integer(col) => col.push(value),
            Self::Real(col) => col.push(value),
        }
    }

    pub fn empty_binary() -> Self {
        Self::Binary(ColElement(Vec::default(), PhantomData::default()))
    }

    pub fn empty_spin() -> Self {
        Self::Spin(ColElement(Vec::default(), PhantomData::default()))
    }

    pub fn empty_integer() -> Self {
        Self::Integer(ColElement(Vec::default(), PhantomData::default()))
    }

    pub fn empty_real() -> Self {
        Self::Real(ColElement(Vec::default(), PhantomData::default()))
    }

    pub fn binary(data: Vec<f64>) -> Self {
        Self::Binary(ColElement(data, PhantomData::default()))
    }

    pub fn spin(data: Vec<f64>) -> Self {
        Self::Spin(ColElement(data, PhantomData::default()))
    }

    pub fn integer(data: Vec<f64>) -> Self {
        Self::Integer(ColElement(data, PhantomData::default()))
    }

    pub fn real(data: Vec<f64>) -> Self {
        Self::Real(ColElement(data, PhantomData::default()))
    }
}

impl<T: NumCast> ColElement<T> {
    pub fn as_t(&self, index: usize) -> T {
        <T as NumCast>::from(self.0[index]).unwrap()
    }
}

impl ColElement<u8> {
    pub fn push(&mut self, value: Assignment) -> LunaModelResult<()> {
        // we need to check that the value
        let msg = match value {
            Assignment::Binary(v) => {
                self.0.push(v as Bias);
                None
            }
            Assignment::Spin(_) => Some("spin"),
            Assignment::Integer(_) => Some("integer"),
            Assignment::Real(_) => Some("real"),
        };
        // msg.ok_or(|| LunaModelError::Dtype(msg.into()))
        // Err(LunaModelError::Dtype("can not push a ".into())),
        // msg.ok()
    }
}
