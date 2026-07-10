use lunamodel_cext_vtable::{LmComparator, LmSense, LmValueSource, LmVtype};
use lunamodel_types::{Comparator, Sense, ValueSource, Vtype};

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

impl LmFrom<ValueSource> for LmValueSource {
    fn lm_from(value: ValueSource) -> Self {
        match value {
            ValueSource::Raw => Self::Raw,
            ValueSource::Obj => Self::Obj,
        }
    }
}

impl LmFrom<LmValueSource> for ValueSource {
    fn lm_from(value: LmValueSource) -> Self {
        match value {
            LmValueSource::Raw => Self::Raw,
            LmValueSource::Obj => Self::Obj,
        }
    }
}

impl LmTryFrom<LmValueSource> for ValueSource {
    fn lm_try_from(value: LmValueSource) -> crate::Result<Self> {
        Ok(Self::lm_from(value))
    }
}

impl LmFrom<Comparator> for LmComparator {
    fn lm_from(value: Comparator) -> Self {
        match value {
            Comparator::Eq => LmComparator::Eq,
            Comparator::Le => LmComparator::Le,
            Comparator::Ge => LmComparator::Ge,
        }
    }
}

impl LmFrom<LmComparator> for Comparator {
    fn lm_from(value: LmComparator) -> Self {
        match value {
            LmComparator::Eq => Comparator::Eq,
            LmComparator::Le => Comparator::Le,
            LmComparator::Ge => Comparator::Ge,
        }
    }
}

impl LmTryFrom<LmComparator> for Comparator {
    fn lm_try_from(value: LmComparator) -> crate::Result<Self> {
        Ok(Self::lm_from(value))
    }
}
