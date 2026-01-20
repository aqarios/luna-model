use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
    ops::Index,
};

use lunamodel_error::{LunaModelError, LunaModelResult};
use num::{NumCast, ToPrimitive};

use lunamodel_types::{Bias, BinaryAssignment, IntegerAssignment, RealAssignment, SpinAssignment};

use crate::traits::FilterByMask;

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Assignment {
    Binary(BinaryAssignment),
    Spin(SpinAssignment),
    Integer(IntegerAssignment),
    Real(RealAssignment),
}

impl Into<Assignment> for u8 {
    fn into(self) -> Assignment {
        Assignment::Binary(self)
    }
}
impl Into<Assignment> for i8 {
    fn into(self) -> Assignment {
        Assignment::Spin(self)
    }
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
    pub fn with(assignment: Assignment) -> Self {
        match assignment {
            Assignment::Binary(v) => Self::binary(vec![v as Bias]),
            Assignment::Spin(v) => Self::spin(vec![v as Bias]),
            Assignment::Integer(v) => Self::integer(vec![v as Bias]),
            Assignment::Real(v) => Self::real(vec![v as Bias]),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Binary(v) => v.0.len(),
            Self::Spin(v) => v.0.len(),
            Self::Integer(v) => v.0.len(),
            Self::Real(v) => v.0.len(),
        }
    }

    pub fn as_assignment(&self, index: usize) -> Assignment {
        match self {
            Self::Binary(col) => Assignment::Binary(col.as_t(index)),
            Self::Spin(col) => Assignment::Spin(col.as_t(index)),
            Self::Integer(col) => Assignment::Integer(col.as_t(index)),
            Self::Real(col) => Assignment::Real(col.as_t(index)),
        }
    }

    pub fn as_assignments(&self) -> Vec<Assignment> {
        match self {
            Self::Binary(col) => (0..self.len())
                .map(|index| Assignment::Binary(col.as_t(index)))
                .collect(),
            Self::Spin(col) => (0..self.len())
                .map(|index| Assignment::Spin(col.as_t(index)))
                .collect(),
            Self::Integer(col) => (0..self.len())
                .map(|index| Assignment::Integer(col.as_t(index)))
                .collect(),

            Self::Real(col) => (0..self.len())
                .map(|index| Assignment::Real(col.as_t(index)))
                .collect(),
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

    pub fn try_push<N: ToPrimitive + Debug>(&mut self, value: N) -> LunaModelResult<()> {
        match self {
            Self::Binary(col) => match <u8 as NumCast>::from(value) {
                None => return Err(LunaModelError::SampleIncompatibleVtype),
                Some(v) => col.push(Assignment::Binary(v)),
            },
            Self::Spin(col) => match <i8 as NumCast>::from(value) {
                None => return Err(LunaModelError::SampleIncompatibleVtype),
                Some(v) => col.push(Assignment::Spin(v)),
            },
            Self::Integer(col) => match <i64 as NumCast>::from(value) {
                None => return Err(LunaModelError::SampleIncompatibleVtype),
                Some(v) => col.push(Assignment::Integer(v)),
            },

            Self::Real(col) => match <f64 as NumCast>::from(value) {
                None => return Err(LunaModelError::SampleIncompatibleVtype),
                Some(v) => col.push(Assignment::Real(v)),
            },
        }
    }

    pub fn remove(&mut self, index: usize) {
        match self {
            Self::Binary(col) => col.remove(index),
            Self::Spin(col) => col.remove(index),
            Self::Integer(col) => col.remove(index),
            Self::Real(col) => col.remove(index),
        }
    }

    pub fn extract(&self, row: usize) -> Self {
        match self {
            Self::Binary(col) => Self::binary(vec![col[row]]),
            Self::Spin(col) => Self::spin(vec![col[row]]),
            Self::Integer(col) => Self::integer(vec![col[row]]),
            Self::Real(col) => Self::real(vec![col[row]]),
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

    pub fn filter_by_mask(&self, mask: &[bool]) -> Self {
        match self {
            Self::Binary(col) => Self::Binary(col.filter_by_mask(mask)),
            Self::Spin(col) => Self::Spin(col.filter_by_mask(mask)),
            Self::Integer(col) => Self::Integer(col.filter_by_mask(mask)),
            Self::Real(col) => Self::Real(col.filter_by_mask(mask)),
        }
    }
}

impl<T: NumCast> ColElement<T> {
    pub fn as_t(&self, index: usize) -> T {
        <T as NumCast>::from(self.0[index]).unwrap()
    }
}

impl<T> ColElement<T> {
    pub fn filter_by_mask(&self, mask: &[bool]) -> Self {
        Self(self.0.filter_by_mask(mask), self.1)
    }

    pub fn remove(&mut self, index: usize) {
        _ = self.0.remove(index);
    }
}

impl<T> Index<usize> for ColElement<T> {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl ColElement<u8> {
    pub fn push(&mut self, value: Assignment) -> LunaModelResult<()> {
        let msg = match value {
            Assignment::Binary(v) => {
                self.0.push(v as Bias);
                None
            }
            Assignment::Spin(_) => Some("spin"),
            Assignment::Integer(_) => Some("integer"),
            Assignment::Real(_) => Some("real"),
        };
        match msg {
            Some(m) => Err(LunaModelError::Dtype(m.into())),
            None => Ok(()),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = u8> {
        self.0.iter().map(|&e| e as u8)
    }

    pub fn data(&self) -> Vec<u8> {
        self.iter().collect()
    }
}

impl ColElement<i8> {
    pub fn push(&mut self, value: Assignment) -> LunaModelResult<()> {
        let msg = match value {
            Assignment::Binary(_) => Some("binary"),
            Assignment::Spin(v) => {
                self.0.push(v as Bias);
                None
            }
            Assignment::Integer(_) => Some("integer"),
            Assignment::Real(_) => Some("real"),
        };
        match msg {
            Some(m) => Err(LunaModelError::Dtype(m.into())),
            None => Ok(()),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = i8> {
        self.0.iter().map(|&e| e as i8)
    }

    pub fn data(&self) -> Vec<i8> {
        self.iter().collect()
    }
}

impl ColElement<i64> {
    pub fn push(&mut self, value: Assignment) -> LunaModelResult<()> {
        let msg = match value {
            Assignment::Binary(_) => Some("binary"),
            Assignment::Spin(_) => Some("spin"),
            Assignment::Integer(v) => {
                self.0.push(v as Bias);
                None
            }

            Assignment::Real(_) => Some("real"),
        };
        match msg {
            Some(m) => Err(LunaModelError::Dtype(m.into())),
            None => Ok(()),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = i64> {
        self.0.iter().map(|&e| e as i64)
    }

    pub fn data(&self) -> Vec<i64> {
        self.iter().collect()
    }
}

impl ColElement<f64> {
    pub fn push(&mut self, value: Assignment) -> LunaModelResult<()> {
        let msg = match value {
            Assignment::Binary(_) => Some("binary"),
            Assignment::Spin(_) => Some("spin"),
            Assignment::Integer(_) => Some("integer"),
            Assignment::Real(v) => {
                self.0.push(v as Bias);
                None
            }
        };
        match msg {
            Some(m) => Err(LunaModelError::Dtype(m.into())),
            None => Ok(()),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = f64> {
        self.0.iter().map(|&v| v)
    }

    pub fn data(&self) -> Vec<f64> {
        self.iter().collect()
    }
}

impl Display for Assignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Binary(b) => write!(f, "{b}"),
            Self::Spin(s) => write!(f, "{s}"),
            Self::Integer(i) => write!(f, "{i}"),
            Self::Real(r) => {
                let s = format!("{r:.}");
                match s.contains(".") {
                    true => write!(f, "{}", s),
                    false => write!(f, "{}.0", s),
                }
            }
        }
    }
}
