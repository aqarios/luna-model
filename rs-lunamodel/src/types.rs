use lunamodel_cext_vtable::{LmSense, LmVtype};
use lunamodel_types::{Sense, Vtype};

use crate::traits::{LmFrom, LmTryFrom};

impl LmFrom<LmVtype> for Vtype {
    fn lm_from(value: LmVtype) -> Self {
        match value {
            LmVtype::Binary => Self::Binary,
            LmVtype::InvertedBinary => Self::InvertedBinary,
            LmVtype::Spin => Self::Spin,
            LmVtype::Integer => Self::Integer,
            LmVtype::Real => Self::Real,
        }
    }
}

impl LmFrom<Vtype> for LmVtype {
    fn lm_from(value: Vtype) -> Self {
        match value {
            Vtype::Binary => Self::Binary,
            Vtype::InvertedBinary => Self::InvertedBinary,
            Vtype::Spin => Self::Spin,
            Vtype::Integer => Self::Integer,
            Vtype::Real => Self::Real,
        }
    }
}

impl LmTryFrom<LmVtype> for Vtype {
    fn lm_try_from(value: LmVtype) -> crate::Result<Self> {
        Ok(Self::lm_from(value))
    }
}

impl LmFrom<Sense> for LmSense {
    fn lm_from(value: Sense) -> Self {
        match value {
            Sense::Min => Self::Min,
            Sense::Max => Self::Max,
        }
    }
}

impl LmFrom<LmSense> for Sense {
    fn lm_from(value: LmSense) -> Self {
        match value {
            LmSense::Min => Self::Min,
            LmSense::Max => Self::Max,
        }
    }
}

impl LmTryFrom<LmSense> for Sense {
    fn lm_try_from(value: LmSense) -> crate::Result<Self> {
        Ok(Self::lm_from(value))
    }
}
