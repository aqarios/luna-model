use crate::{
    core::{
        term::types::{OneVarTerm, OneVarTermConstruction},
        VarId,
    },
    types::Bias,
};
use std::{
    cmp::Ordering,
    isize,
    ops::{Index, IndexMut, MulAssign, Neg},
    slice::IterMut,
};

#[derive(Clone, Debug)]
pub struct Linear {
    biases: Vec<OneVarTerm>,
    max_idx: isize,
    default_bias: Bias,
}

impl Linear {
    pub fn default() -> Self {
        Self {
            biases: Vec::new(),
            max_idx: -1,
            default_bias: Bias::default(),
        }
    }

    pub fn new(biases: Vec<Bias>) -> Self {
        let mut max_idx: isize = -1;
        let biases = biases
            .into_iter()
            .enumerate()
            .filter(|(_, bias)| *bias != Bias::default())
            .map(|(idx, bias)| {
                max_idx = idx as isize;
                OneVarTerm::new(VarId(idx as u32), bias)
            })
            .collect();
        Self {
            max_idx: max_idx,
            biases,
            default_bias: Bias::default(),
        }
    }

    pub fn new_from_weighted_variable(var: usize, bias: Bias) -> Self {
        let mut out = Self::default();
        out.max_idx = var as isize;
        if bias != Bias::default() {
            out.biases.push(OneVarTerm::new(VarId(var as u32), bias));
        }
        out
    }

    pub fn new_from_variables(lhs: (usize, Bias), rhs: (usize, Bias)) -> Self {
        let mut out = Self::default();
        if lhs.0 < rhs.0 {
            out.max_idx = rhs.0 as isize;
            out.biases.push(OneVarTerm::new(VarId(lhs.0 as u32), lhs.1));
            out.biases.push(OneVarTerm::new(VarId(rhs.0 as u32), rhs.1));
        } else {
            out.max_idx = lhs.0 as isize;
            out.biases.push(OneVarTerm::new(VarId(rhs.0 as u32), rhs.1));
            out.biases.push(OneVarTerm::new(VarId(lhs.0 as u32), lhs.1));
        }
        out
    }

    pub fn to_vec(&self, length: usize) -> Vec<Bias> {
        // let length = length.unwrap_or_else(|| (self.max_idx + 1) as usize);
        if length == 0 {
            return Vec::default();
        }

        // let mut linear = vec![0.0; (self.max_idx + 1) as usize];
        let length = length.max((self.max_idx + 1) as usize);
        let mut linear = vec![0.0; length];
        for (u, bias) in self.iter() {
            linear[u] = bias;
        }
        linear
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, Bias)> + '_ {
        self.biases.iter().map(|t| (t.index.0 as usize, t.bias))
    }

    pub fn iter_mut(&mut self) -> IterMut<OneVarTerm> {
        let mutvec: &mut Vec<OneVarTerm> = self.biases.as_mut();
        mutvec.iter_mut()
    }

    pub fn len(&self) -> usize {
        self.biases.len()
    }

    // todo(team): remove
    pub fn resize(&mut self, _: usize) {
        // self.biases.resize_with(new_len, Bias::default);
    }

    pub fn is_zero(&self) -> bool {
        let mut all_zero = true;
        for &t in self.biases.iter() {
            all_zero &= t.bias == Bias::default();
        }
        all_zero
    }

    pub fn add(&mut self, index: usize, bias: Bias) -> bool {
        let pos = self
            .biases
            .binary_search_by(|term| {
                term.index
                    .partial_cmp(&index.into())
                    .unwrap_or(Ordering::Equal)
            })
            .unwrap_or_else(|insert_pos| insert_pos);
        if pos == self.biases.len() {
            self.max_idx = index as isize;
            self.biases.push(OneVarTerm::new(index.into(), bias));
            true
        } else if self.biases[pos].index != index.into() {
            self.biases.insert(pos, OneVarTerm::new(index.into(), bias));
            true
        } else {
            self.biases[pos].bias += bias;
            false
        }
    }
}

impl MulAssign<Bias> for Linear {
    fn mul_assign(&mut self, rhs: Bias) {
        for b in self.biases.iter_mut() {
            b.bias *= rhs;
        }
    }
}

impl From<&Vec<Bias>> for Linear {
    fn from(value: &Vec<Bias>) -> Self {
        Self::new(value.to_vec())
    }
}

impl Index<usize> for Linear {
    type Output = Bias;

    fn index(&self, index: usize) -> &Self::Output {
        let pos = self.biases.binary_search_by(|term| {
            term.index
                .partial_cmp(&index.into())
                .unwrap_or(Ordering::Equal)
        });
        match pos {
            Ok(p) => &self.biases[p].bias,
            Err(_) => &self.default_bias,
        }
    }
}

impl IndexMut<usize> for Linear {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let pos = self
            .biases
            .binary_search_by(|term| {
                term.index
                    .partial_cmp(&index.into())
                    .unwrap_or(Ordering::Equal)
            })
            .unwrap_or_else(|insert_pos| insert_pos);
        if pos == self.biases.len() {
            self.max_idx = index as isize;
            self.biases.push(OneVarTerm::new_default(index.into()))
        } else if self.biases[pos].index != index.into() {
            self.biases
                .insert(pos, OneVarTerm::new_default(index.into()));
        }

        &mut self.biases[pos].bias
    }
}

impl PartialEq for Linear {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }
        self.biases == other.biases
    }
}

impl Linear {
    fn negate(&self) -> Self {
        // Linear::new(self.biases.iter().map(|t| -t.bias).collect())
        let out = Self {
            biases: self
                .biases
                .iter()
                .map(|t| OneVarTerm::new(t.index, -t.bias))
                .collect(),
            max_idx: self.max_idx,
            default_bias: Bias::default(),
        };
        out
    }
}

impl Neg for Linear {
    type Output = Linear;
    fn neg(self) -> Self::Output {
        self.negate()
    }
}

impl Neg for &Linear {
    type Output = Linear;
    fn neg(self) -> Self::Output {
        self.negate()
    }
}
