use num::{NumCast, ToPrimitive};

use crate::{
    core::{VarAssignment, Vtype},
    errors::{SampleColCreationErr, SampleIncompatibleVtypeErr},
    types::{
        BinaryAssignmentType, IntegerAssignmentType, RealAssignmentType, SpinAssignmentType,
        VarIndex,
    },
};

#[derive(Debug, Clone, PartialEq)]
pub struct ColElement<T> {
    // todo: is this var id really needed?
    pub varid: VarIndex,
    pub data: Vec<T>,
}

impl<T> ColElement<T> {
    pub fn new(varid: VarIndex, data: Vec<T>) -> Self {
        Self { varid, data }
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }

    pub fn get(&self, idx: usize) -> Option<&T> {
        self.data.get(idx)
    }
}

/// The different assignments to a variable in the single samples
#[derive(Debug, Clone, PartialEq)]
pub enum Column {
    Binary(ColElement<BinaryAssignmentType>),
    Spin(ColElement<SpinAssignmentType>),
    Integer(ColElement<IntegerAssignmentType>),
    Real(ColElement<RealAssignmentType>),
}

impl Column {
    pub fn get(&self, idx: usize) -> Option<VarAssignment> {
        match self {
            Self::Binary(col) => col.get(idx).map(|&x| VarAssignment::Binary(x)),
            Self::Spin(col) => col.get(idx).map(|&x| VarAssignment::Spin(x)),
            Self::Integer(col) => col.get(idx).map(|&x| VarAssignment::Integer(x)),
            Self::Real(col) => col.get(idx).map(|&x| VarAssignment::Real(x)),
        }
    }
}

impl Column {
    pub fn push<N: ToPrimitive>(
        &mut self,
        assignment: N,
    ) -> Result<(), SampleIncompatibleVtypeErr> {
        match self {
            Self::Binary(xs) => match <BinaryAssignmentType as NumCast>::from(assignment) {
                None => return Err(SampleIncompatibleVtypeErr),
                Some(x) => {
                    xs.push(x);
                }
            },
            Self::Spin(xs) => match <SpinAssignmentType as NumCast>::from(assignment) {
                None => return Err(SampleIncompatibleVtypeErr),
                Some(x) => {
                    xs.push(x);
                }
            },
            Self::Integer(xs) => match <IntegerAssignmentType as NumCast>::from(assignment) {
                None => return Err(SampleIncompatibleVtypeErr),
                Some(x) => {
                    xs.push(x);
                }
            },
            Self::Real(xs) => match <RealAssignmentType as NumCast>::from(assignment) {
                None => return Err(SampleIncompatibleVtypeErr),
                Some(x) => {
                    xs.push(x);
                }
            },
        };
        Ok(())
    }

    pub fn try_new<N: NumCast + Copy>(
        data: &[N],
        varid: VarIndex,
        vtype: Vtype,
    ) -> Result<Column, SampleColCreationErr> {
        match vtype {
            Vtype::Binary => Ok(Column::Binary(Self::try_make_samplecol_elem(varid, data)?)),
            Vtype::Spin => Ok(Column::Spin(Self::try_make_samplecol_elem(varid, data)?)),
            Vtype::Real => Ok(Column::Real(Self::try_make_samplecol_elem(varid, data)?)),
            Vtype::Integer => Ok(Column::Integer(Self::try_make_samplecol_elem(varid, data)?)),
            Vtype::__Ghost => Err(SampleColCreationErr::new(
                "cannot create a sample column for ghost variables.",
            )),
        }
    }
    fn try_make_samplecol_elem<T: NumCast, N: NumCast + Copy>(
        varid: VarIndex,
        data: &[N],
    ) -> Result<ColElement<T>, SampleColCreationErr> {
        Ok(ColElement::new(
            varid,
            Self::try_make_sample_col_element_contents(data)?,
        ))
    }

    fn try_make_sample_col_element_contents<T: NumCast, N: NumCast + Copy>(
        data: &[N],
    ) -> Result<Vec<T>, SampleColCreationErr> {
        data.iter()
            .map(|e| <T as NumCast>::from(*e).ok_or_else(|| SampleColCreationErr::default()))
            .collect()
    }

    pub fn new_binary(varid: VarIndex, data: Vec<BinaryAssignmentType>) -> Column {
        Column::Binary(ColElement::new(varid, data))
    }

    pub fn new_spin(varid: VarIndex, data: Vec<SpinAssignmentType>) -> Column {
        Column::Spin(ColElement::new(varid, data))
    }

    pub fn new_integer(varid: VarIndex, data: Vec<IntegerAssignmentType>) -> Column {
        Column::Integer(ColElement::new(varid, data))
    }

    pub fn new_real(varid: VarIndex, data: Vec<RealAssignmentType>) -> Column {
        Column::Real(ColElement::new(varid, data))
    }
}
