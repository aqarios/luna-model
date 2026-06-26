use crate::LmSense;
use luna_model::types::{Sense, Vtype};
use lunamodel_cext_vtable::LmVtype;

pub fn sense_from_c(sense: LmSense) -> Sense {
    match sense {
        LmSense::Min => Sense::Min,
        LmSense::Max => Sense::Max,
    }
}

pub fn sense_to_c(sense: Sense) -> LmSense {
    match sense {
        Sense::Min => LmSense::Min,
        Sense::Max => LmSense::Max,
    }
}

pub fn vtype_to_c(vtype: Vtype) -> LmVtype {
    match vtype {
        Vtype::Binary => LmVtype::Binary,
        Vtype::InvertedBinary => LmVtype::InvertedBinary,
        Vtype::Spin => LmVtype::Spin,
        Vtype::Integer => LmVtype::Integer,
        Vtype::Real => LmVtype::Real,
    }
}

pub fn vtype_from_c(vtype: LmVtype) -> Vtype {
    match vtype {
        LmVtype::Binary => Vtype::Binary,
        LmVtype::InvertedBinary => Vtype::InvertedBinary,
        LmVtype::Spin => Vtype::Spin,
        LmVtype::Integer => Vtype::Integer,
        LmVtype::Real => Vtype::Real,
    }
}
