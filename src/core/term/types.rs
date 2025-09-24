use std::ops::Neg;

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

impl Neg for OneVarTerm {
    type Output = OneVarTerm;

    fn neg(self) -> Self::Output {
        OneVarTerm::new(self.index, -self.bias)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct TwoVarTerm {
    pub index: VarIndex,
    pub neighborhood: Vec<OneVarTerm>,
}

impl TwoVarTerm {
    pub fn is_empty(&self) -> bool {
        self.neighborhood.is_empty()
    }

    pub fn push(&mut self, neighbor: OneVarTerm) {
        self.neighborhood.push(neighbor);
    }

    pub fn last(&self) -> Option<&OneVarTerm> {
        self.neighborhood.last()
    }
}

pub trait TwoVarTermConstruction {
    fn new(index: VarIndex, neighborhood: Vec<OneVarTerm>) -> Self;
    fn new_default(index: VarIndex) -> Self;
}

impl TwoVarTermConstruction for TwoVarTerm {
    fn new(index: VarIndex, neighborhood: Vec<OneVarTerm>) -> Self {
        Self {
            index,
            neighborhood,
        }
    }

    fn new_default(index: VarIndex) -> Self {
        Self {
            index,
            neighborhood: Vec::default(),
        }
    }
}
