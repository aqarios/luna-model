use lunamodel_cext_vtable::LmVtype;
use lunamodel_types::Vtype;

use crate::traits::LmFrom;

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
