use lunamodel_types::{Bias, VarIdx};
use std::ops::Neg;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct OneVarTerm {
    pub(super) idx: VarIdx,
    pub(super) bias: Bias,
}

impl OneVarTerm {
    pub fn new(idx: VarIdx, bias: Bias) -> Self {
        Self { idx, bias }
    }

    pub fn default(idx: VarIdx) -> Self {
        Self {
            idx,
            bias: Bias::default(),
        }
    }
}

impl Neg for OneVarTerm {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(self.idx, -self.bias)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct TwoVarTerm {
    pub(super) idx: VarIdx,
    pub(super) neighborhood: Vec<OneVarTerm>,
}

impl TwoVarTerm {
    pub fn new(idx: VarIdx, neighborhood: Vec<OneVarTerm>) -> Self {
        Self { idx, neighborhood }
    }

    pub fn empty(idx: VarIdx) -> Self {
        Self {
            idx,
            neighborhood: Vec::default(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.neighborhood.is_empty()
    }

    pub fn push(&mut self, neighbor: OneVarTerm) -> &mut Self {
        self.neighborhood.push(neighbor);
        self
    }

    pub fn last(&self) -> Option<&OneVarTerm> {
        self.neighborhood.last()
    }
}
