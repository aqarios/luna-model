use luna_model::types::{Bound, Comparator, Ctype, Sense, Vtype};

use crate::{LmBound, LmBoundTag, LmComparator, LmCtype, LmSense, LmVtype};

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

pub fn ctype_to_c(ctype: Ctype) -> LmCtype {
    match ctype {
        Ctype::Unconstrained => LmCtype::Unconstrained,
        Ctype::Equality => LmCtype::Equality,
        Ctype::Inequality => LmCtype::Inequality,
        Ctype::LessEqual => LmCtype::LessEqual,
        Ctype::GreaterEqual => LmCtype::GreaterEqual,
    }
}

pub fn ctype_from_c(ctype: LmCtype) -> Ctype {
    match ctype {
        LmCtype::Unconstrained => Ctype::Unconstrained,
        LmCtype::Equality => Ctype::Equality,
        LmCtype::Inequality => Ctype::Inequality,
        LmCtype::LessEqual => Ctype::LessEqual,
        LmCtype::GreaterEqual => Ctype::GreaterEqual,
    }
}

pub fn cmp_to_c(cmp: Comparator) -> LmComparator {
    match cmp {
        Comparator::Eq => LmComparator::Eq,
        Comparator::Le => LmComparator::Le,
        Comparator::Ge => LmComparator::Ge,
    }
}

pub fn cmp_from_c(cmp: LmComparator) -> Comparator {
    match cmp {
        LmComparator::Eq => Comparator::Eq,
        LmComparator::Le => Comparator::Le,
        LmComparator::Ge => Comparator::Ge,
    }
}

pub fn bound_from_c(bound: LmBound) -> Bound {
    match bound.tag {
        LmBoundTag::Unbounded => Bound::Unbounded,
        LmBoundTag::Bounded => Bound::Bounded(bound.value),
    }
}

pub fn bound_to_c(bound: Bound) -> LmBound {
    match bound {
        Bound::Unbounded => LmBound {
            tag: LmBoundTag::Unbounded,
            value: 0.0,
        },
        Bound::Bounded(val) => LmBound {
            tag: LmBoundTag::Bounded,
            value: val,
        },
    }
}
