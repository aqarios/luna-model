use crate::LmSense;
use luna_model::types::Sense;

pub fn sense_from_c(sense: LmSense) -> Sense {
    match sense {
        LmSense::Min => Sense::Min,
        LmSense::Max => Sense::Max,
    }
}
