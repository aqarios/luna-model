//! Value-level sparse term helpers matching `core::expression::term::types`.

use std::{cmp::Ordering, ops::Neg};

use crate::{Bias, VarIdx};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct OneVarTerm {
    idx: VarIdx,
    bias: Bias,
}

impl OneVarTerm {
    pub fn new(idx: VarIdx, bias: Bias) -> Self {
        Self { idx, bias }
    }
    pub fn default(idx: VarIdx) -> Self {
        Self::new(idx, Bias::default())
    }
}

impl Neg for OneVarTerm {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(self.idx, -self.bias)
    }
}

#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct Neighborhood(pub Vec<OneVarTerm>);

impl Neighborhood {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> Self {
        Self(Vec::new())
    }
    pub fn is_zero(&self) -> bool {
        self.iter().map(|(_, bias)| bias).sum::<Bias>() == 0.0
    }
    pub fn iter(&self) -> impl Iterator<Item = (VarIdx, Bias)> + '_ {
        self.0.iter().map(|term| (term.idx, term.bias))
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (VarIdx, &mut Bias)> + '_ {
        self.0.iter_mut().map(|term| (term.idx, &mut term.bias))
    }
    pub fn push_back(&mut self, var: VarIdx, bias: Bias) -> &mut Self {
        self.0.push(OneVarTerm::new(var, bias));
        self
    }
    pub fn push_back_empty(&mut self, var: VarIdx) -> &mut Self {
        self.push_back(var, 0.0)
    }
    pub fn insert(&mut self, pos: usize, var: VarIdx, bias: Bias) -> &mut Self {
        self.0.insert(pos, OneVarTerm::new(var, bias));
        self
    }
    pub fn insert_empty(&mut self, pos: usize, var: VarIdx) -> &mut Self {
        self.insert(pos, var, 0.0)
    }
    pub fn find(&self, needle: VarIdx) -> std::result::Result<usize, usize> {
        self.0
            .binary_search_by(|term| term.idx.partial_cmp(&needle).unwrap_or(Ordering::Equal))
    }
}

impl std::ops::Deref for Neighborhood {
    type Target = Vec<OneVarTerm>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Neighborhood {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromIterator<OneVarTerm> for Neighborhood {
    fn from_iter<T: IntoIterator<Item = OneVarTerm>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl FromIterator<(VarIdx, Bias)> for Neighborhood {
    fn from_iter<T: IntoIterator<Item = (VarIdx, Bias)>>(iter: T) -> Self {
        Self(
            iter.into_iter()
                .map(|(idx, bias)| OneVarTerm::new(idx, bias))
                .collect(),
        )
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct TwoVarTerm {
    idx: VarIdx,
    neighborhood: Neighborhood,
}

impl TwoVarTerm {
    pub fn new(idx: VarIdx, neighborhood: Neighborhood) -> Self {
        Self { idx, neighborhood }
    }
}
