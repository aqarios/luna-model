use crate::types::{Bias, VarIndex};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct OneVarTerm {
    pub index: VarIndex,
    pub bias: Bias,
}

pub type SizeType = usize;

pub trait OneVarTermConstruction {
    fn new(index: VarIndex, bias: Bias) -> Self;
    fn new_default(index: VarIndex) -> Self;
}

impl OneVarTermConstruction for OneVarTerm {
    fn new(index: VarIndex, bias: Bias) -> Self {
        Self { index, bias }
    }

    fn new_default(index: VarIndex) -> Self {
        Self {
            index,
            bias: Bias::default(),
        }
    }
}
