use super::types::OneVarTerm;
use lunamodel_types::{Bias, DEFAULT_BIAS, VarIdx};
use std::cmp::Ordering;
use std::ops::{AddAssign, Index, IndexMut, MulAssign, Neg};

// neighborhood of Quadratic two var term and linear biases is the exact same thing.
// We can reduce code complexity and duplications a lot, if we combine them to a single, unified
// datatype.

#[derive(Debug, Clone)]
pub struct Linear {
    /// Linear [biases](Self::biases) for each [VarIdx] as a dynamically growing array.
    biases: Vec<OneVarTerm>,
}

impl Linear {
    pub fn default() -> Self {
        Self {
            biases: Vec::default(),
            // hidx: HIdx::None,
        }
    }

    // // This seems too unsafe to be used as a public creation function.
    // // We don't have guarantees that:
    // //  - the passed biases are sorted,
    // //  - each position is eq. to the VarIdx
    // pub fn new(biases: &[Bias]) -> Self {
    //     _ = biases;
    //     todo!("do we really need this? or shouldn't it be a &[(VarIdx, Bias)]?")
    // }

    pub fn len(&self) -> usize {
        self.biases.len()
    }

    pub fn is_zero(&self) -> bool {
        let sum: Bias = self.biases.iter().map(|t| t.bias).sum();
        Bias::default() == sum
    }

    pub fn for_var(var: VarIdx, bias: Bias) -> Self {
        let mut out = Self::default();
        match bias == Bias::default() {
            true => (),
            false => _ = out.push_back(var, bias),
        };
        out
    }

    pub fn for_vars(var_a: (VarIdx, Bias), var_b: (VarIdx, Bias)) -> Self {
        if var_a.0 == var_b.0 {
            Self::for_var(var_a.0, var_a.1 + var_b.1)
        } else if var_a.0 < var_b.0 {
            let mut out = Self::default();
            _ = out.push_back(var_a.0, var_a.1).push_back(var_b.0, var_b.1);
            out
        } else {
            let mut out = Self::default();
            _ = out.push_back(var_b.0, var_b.1).push_back(var_a.0, var_a.1);
            out
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (VarIdx, Bias)> {
        self.biases.iter().map(|t| (t.idx, t.bias))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (VarIdx, &mut Bias)> {
        let mvec: &mut Vec<OneVarTerm> = self.biases.as_mut();
        mvec.iter_mut().map(|t| (t.idx, &mut t.bias))
    }

    fn push_back(&mut self, var: VarIdx, bias: Bias) -> &mut Self {
        self.biases.push(OneVarTerm::new(var, bias));
        self
    }

    fn insert(&mut self, pos: usize, var: VarIdx, bias: Bias) -> &mut Self {
        self.biases.insert(pos, OneVarTerm::new(var, bias));
        self
    }

    // again, do we really need this??
    // fn add(&mut self, idx: VarIdx, bias: Bias) -> bool {}

    pub(super) fn find(hay: &[OneVarTerm], needle: VarIdx) -> Result<usize, usize> {
        hay.binary_search_by(|t| t.idx.partial_cmp(&needle).unwrap_or(Ordering::Equal))
    }
}

impl AddAssign<(VarIdx, Bias)> for Linear {
    fn add_assign(&mut self, rhs: (VarIdx, Bias)) {
        let pos = Self::find(&self.biases, rhs.0).unwrap_or_else(|l| l);
        if pos == self.len() {
            self.push_back(rhs.0, rhs.1);
        } else if self.biases[pos].idx != rhs.0 {
            self.insert(pos, rhs.0, rhs.1);
        } else {
            self.biases[pos].bias += rhs.1;
        }
    }
}

impl MulAssign<Bias> for Linear {
    fn mul_assign(&mut self, rhs: Bias) {
        self.iter_mut().for_each(|(_, bias)| *bias *= rhs);
    }
}

impl Index<VarIdx> for Linear {
    type Output = Bias;

    fn index(&self, index: VarIdx) -> &Self::Output {
        match Self::find(&self.biases, index).ok() {
            Some(p) => &self.biases[p].bias,
            None => &DEFAULT_BIAS,
        }
    }
}

impl IndexMut<VarIdx> for Linear {
    fn index_mut(&mut self, index: VarIdx) -> &mut Self::Output {
        let pos = Self::find(&self.biases, index).unwrap_or_else(|l| l);
        if pos == self.len() {
            self.push_back(index, Bias::default());
        } else if self.biases[pos].idx != index {
            self.insert(pos, index, Bias::default());
        }
        &mut self.biases[pos].bias
    }
}

impl PartialEq for Linear {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.biases == other.biases
    }
}

impl Neg for Linear {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            biases: self
                .biases
                .iter()
                .map(|t| OneVarTerm::new(t.idx, -t.bias))
                .collect(),
        }
    }
}
