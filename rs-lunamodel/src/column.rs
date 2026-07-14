use std::{fmt::Debug, marker::PhantomData, ops::Index};

use lunamodel_cext_vtable::LmColumn;
use lunamodel_types::{
    Bias, BinaryAssignment, IntegerAssignment, RealAssignment, SpinAssignment, Vtype,
};
use num::{NumCast, ToPrimitive};

use crate::{Assignment, Error, Result, Status, api::api, traits::LmTryFrom};

/// Typed column storage backed by raw `f64` values.
#[derive(Debug, Clone, PartialEq)]
pub struct ColElement<T>(pub Vec<f64>, PhantomData<T>);

/// One variable column within a solution.
#[derive(Debug, Clone, PartialEq)]
pub enum Column {
    Binary(ColElement<BinaryAssignment>),
    Spin(ColElement<SpinAssignment>),
    Integer(ColElement<IntegerAssignment>),
    Real(ColElement<RealAssignment>),
}

struct RawColumn(*mut LmColumn);

impl RawColumn {
    fn as_ptr(&self) -> *const LmColumn {
        self.0
    }
}

impl Drop for RawColumn {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { (api().col_free)(self.0) };
        }
    }
}

impl LmTryFrom<*mut LmColumn> for Column {
    fn lm_try_from(handle: *mut LmColumn) -> Result<Self> {
        if handle.is_null() {
            return Err(invalid("the LunaModel host returned a null column handle"));
        }
        let handle = RawColumn(handle);
        let vtype: Vtype = out!(col_vtype(@handle))?;
        let _is_empty: bool = out!(col_is_empty(@handle))?;
        let len: usize = out!(col_len(@handle))?;
        let mut values = Vec::with_capacity(len);
        for idx in 0..len {
            let assignment: Assignment = out!(col_assignment(@handle, idx))?;
            values.push(assignment.value());
        }
        Ok(match vtype {
            Vtype::Binary | Vtype::InvertedBinary => Self::binary(values),
            Vtype::Spin => Self::spin(values),
            Vtype::Integer => Self::integer(values),
            Vtype::Real => Self::real(values),
        })
    }
}

impl Index<usize> for Column {
    type Output = Bias;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values()[index]
    }
}

impl Column {
    pub fn with(assignment: Assignment) -> Self {
        match assignment {
            Assignment::Binary(v) => Self::binary(vec![v as Bias]),
            Assignment::Spin(v) => Self::spin(vec![v as Bias]),
            Assignment::Integer(v) => Self::integer(vec![v as Bias]),
            Assignment::Real(v) => Self::real(vec![v]),
        }
    }

    pub fn len(&self) -> usize {
        self.values().len()
    }

    pub fn is_empty(&self) -> bool {
        self.values().is_empty()
    }

    pub fn as_assignment(&self, index: usize) -> Assignment {
        match self {
            Self::Binary(col) => Assignment::Binary(col.as_t(index)),
            Self::Spin(col) => Assignment::Spin(col.as_t(index)),
            Self::Integer(col) => Assignment::Integer(col.as_t(index)),
            Self::Real(col) => Assignment::Real(col.as_t(index)),
        }
    }

    /// Fallible compatibility alias for the former opaque wrapper API.
    pub fn assignment(&self, index: usize) -> Result<Assignment> {
        if index < self.len() {
            Ok(self.as_assignment(index))
        } else {
            Err(invalid(format!("column index {index} is out of bounds")))
        }
    }

    pub fn as_assignments(&self) -> Vec<Assignment> {
        (0..self.len()).map(|idx| self.as_assignment(idx)).collect()
    }

    pub fn push(&mut self, value: Assignment) -> Result<()> {
        match self {
            Self::Binary(col) => col.push(value),
            Self::Spin(col) => col.push(value),
            Self::Integer(col) => col.push(value),
            Self::Real(col) => col.push(value),
        }
    }

    pub fn try_push<N: ToPrimitive + Copy + Debug>(
        &mut self,
        value: N,
        tol: Option<f64>,
    ) -> Result<()> {
        let tol = validate_tol(tol)?;
        let value = value
            .to_f64()
            .filter(|v| v.is_finite())
            .ok_or_else(|| invalid("sample value is not representable as a finite f64"))?;
        match self {
            Self::Binary(col) => {
                let v = near_integral::<u8>(value, tol)?;
                if v > 1 {
                    return Err(invalid("binary samples must be 0 or 1"));
                }
                col.push(Assignment::Binary(v))
            }
            Self::Spin(col) => {
                let v = near_integral::<i8>(value, tol)?;
                if v != -1 && v != 1 {
                    return Err(invalid("spin samples must be -1 or 1"));
                }
                col.push(Assignment::Spin(v))
            }
            Self::Integer(col) => col.push(Assignment::Integer(near_integral(value, tol)?)),
            Self::Real(col) => col.push(Assignment::Real(value)),
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
        Self::binary(Vec::new())
    }
    pub fn empty_spin() -> Self {
        Self::spin(Vec::new())
    }
    pub fn empty_integer() -> Self {
        Self::integer(Vec::new())
    }
    pub fn empty_real() -> Self {
        Self::real(Vec::new())
    }
    pub fn binary(data: Vec<f64>) -> Self {
        Self::Binary(ColElement(data, PhantomData))
    }
    pub fn spin(data: Vec<f64>) -> Self {
        Self::Spin(ColElement(data, PhantomData))
    }
    pub fn integer(data: Vec<f64>) -> Self {
        Self::Integer(ColElement(data, PhantomData))
    }
    pub fn real(data: Vec<f64>) -> Self {
        Self::Real(ColElement(data, PhantomData))
    }

    pub fn filter_by_mask(&self, mask: &[bool]) -> Self {
        match self {
            Self::Binary(col) => Self::Binary(col.filter_by_mask(mask)),
            Self::Spin(col) => Self::Spin(col.filter_by_mask(mask)),
            Self::Integer(col) => Self::Integer(col.filter_by_mask(mask)),
            Self::Real(col) => Self::Real(col.filter_by_mask(mask)),
        }
    }

    pub fn vtype(&self) -> Vtype {
        match self {
            Self::Binary(_) => Vtype::Binary,
            Self::Spin(_) => Vtype::Spin,
            Self::Integer(_) => Vtype::Integer,
            Self::Real(_) => Vtype::Real,
        }
    }

    pub fn values(&self) -> &[f64] {
        match self {
            Self::Binary(v) => &v.0,
            Self::Spin(v) => &v.0,
            Self::Integer(v) => &v.0,
            Self::Real(v) => &v.0,
        }
    }
}

impl<T: NumCast> ColElement<T> {
    pub fn as_t(&self, index: usize) -> T {
        NumCast::from(self.0[index]).expect("stored column value has the declared type")
    }
}

impl<T> ColElement<T> {
    pub fn filter_by_mask(&self, mask: &[bool]) -> Self {
        Self(
            self.0
                .iter()
                .zip(mask)
                .filter_map(|(&value, &keep)| keep.then_some(value))
                .collect(),
            PhantomData,
        )
    }

    pub fn remove(&mut self, index: usize) {
        self.0.remove(index);
    }
}

impl<T> Index<usize> for ColElement<T> {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

macro_rules! typed_col {
    ($ty:ty, $variant:ident) => {
        impl ColElement<$ty> {
            pub fn push(&mut self, value: Assignment) -> Result<()> {
                match value {
                    Assignment::$variant(value) => {
                        self.0.push(value as f64);
                        Ok(())
                    }
                    _ => Err(invalid(concat!(
                        "assignment type does not match ",
                        stringify!($variant),
                        " column"
                    ))),
                }
            }
            pub fn iter(&self) -> impl Iterator<Item = $ty> + '_ {
                self.0.iter().map(|&value| {
                    NumCast::from(value).expect("stored column value has the declared type")
                })
            }
            pub fn data(&self) -> Vec<$ty> {
                self.iter().collect()
            }
        }
    };
}

typed_col!(u8, Binary);
typed_col!(i8, Spin);
typed_col!(i64, Integer);
typed_col!(f64, Real);

fn invalid(message: impl Into<String>) -> Error {
    Error {
        status: Status::InvalidArgument,
        message: message.into(),
    }
}

fn validate_tol(tol: Option<f64>) -> Result<f64> {
    let tol = tol.unwrap_or(1e-6);
    if tol.is_finite() && (0.0..1.0).contains(&tol) {
        Ok(tol)
    } else {
        Err(invalid("tol must be in [0.0, 1.0)"))
    }
}

fn near_integral<T: NumCast>(value: f64, tol: f64) -> Result<T> {
    let rounded = value.round();
    let effective_tol = tol + f64::EPSILON * value.abs().max(1.0);
    if (value - rounded).abs() > effective_tol {
        return Err(invalid(format!(
            "{value} is not sufficiently close to an integer"
        )));
    }
    NumCast::from(rounded).ok_or_else(|| invalid(format!("{value} is outside the target range")))
}
