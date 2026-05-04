//! Sparse storage for linear expression terms.

use lunamodel_types::{Bias, DEFAULT_BIAS, VarIdx};

use crate::traits::Editable;

use super::types::Neighborhood;

use std::ops::{AddAssign, Index, IndexMut, Mul, MulAssign, Neg};

/// Sparse storage for linear expression terms.
///
/// Internally this reuses `Neighborhood` because a linear expression is just a
/// sorted collection of `(variable, bias)` pairs.
#[derive(Debug, Clone, Default)]
pub struct Linear {
    /// Linear [biases](Self::biases) for each [VarIdx] as a dynamically growing array.
    biases: Neighborhood,
}
impl Editable for Linear {}

impl Linear {
    /// Returns the number of explicitly stored variables.
    pub fn len(&self) -> usize {
        self.biases.len()
    }

    /// Returns `true` if all stored biases sum to zero.
    pub fn is_zero(&self) -> bool {
        self.biases.is_zero()
    }

    /// Returns `true` if the storage contains no explicit terms.
    pub fn is_empty(&self) -> bool {
        self.biases.is_empty()
    }

    /// Creates a linear term set containing a single variable contribution.
    pub fn for_var(var: VarIdx, bias: Bias) -> Self {
        let mut out = Self::default();
        match bias == Bias::default() {
            true => (),
            false => _ = out.push_back(var, bias),
        };
        out
    }

    /// Creates a linear term set from two variable contributions.
    ///
    /// If both entries address the same variable, their biases are combined.
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

    /// Iterates over `(variable, bias)` pairs.
    pub fn iter(&self) -> impl Iterator<Item = (VarIdx, Bias)> {
        self.biases.iter()
    }

    /// Iterates mutably over `(variable, bias)` pairs.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (VarIdx, &mut Bias)> {
        self.biases.iter_mut()
    }

    /// Appends a term at the end of the sorted storage.
    fn push_back(&mut self, var: VarIdx, bias: Bias) -> &mut Self {
        self.biases.push_back(var, bias);
        self
    }

    /// Inserts a term at a known sorted position.
    fn insert(&mut self, pos: usize, var: VarIdx, bias: Bias) -> &mut Self {
        self.biases.insert(pos, var, bias);
        self
    }
}

impl AddAssign<(VarIdx, Bias)> for Linear {
    /// Adds a single `(variable, bias)` contribution.
    fn add_assign(&mut self, rhs: (VarIdx, Bias)) {
        let (u, b) = rhs;
        if b == Bias::default() {
            return;
        }
        let pos = self.biases.find(u).unwrap_or_else(|l| l);
        if pos == self.len() {
            self.push_back(u, b);
        } else if self.biases[pos].idx != u {
            self.insert(pos, u, b);
        } else {
            self.biases[pos].bias += b;
        }
    }
}

impl AddAssign<&Linear> for Linear {
    /// Adds all terms from another linear storage.
    fn add_assign(&mut self, rhs: &Linear) {
        for (idx, bias) in rhs.iter() {
            *self += (idx, bias);
        }
    }
}

impl AddAssign<Linear> for Linear {
    /// Adds all terms from another linear storage.
    fn add_assign(&mut self, rhs: Linear) {
        self.add_assign(&rhs)
    }
}

impl MulAssign<Bias> for Linear {
    /// Scales all stored biases by `rhs`.
    fn mul_assign(&mut self, rhs: Bias) {
        if rhs == Bias::default() {
            *self = Self::default();
            return;
        }
        self.iter_mut().for_each(|(_, bias)| *bias *= rhs);
    }
}

impl Mul<Bias> for Linear {
    type Output = Self;

    /// Returns a scaled copy of the linear storage.
    fn mul(mut self, rhs: Bias) -> Self::Output {
        self *= rhs;
        self
    }
}

impl Index<VarIdx> for Linear {
    type Output = Bias;

    /// Returns the bias for a variable, defaulting to zero when absent.
    fn index(&self, index: VarIdx) -> &Self::Output {
        match self.biases.find(index).ok() {
            Some(p) => &self.biases[p].bias,
            None => &DEFAULT_BIAS,
        }
    }
}

impl IndexMut<VarIdx> for Linear {
    /// Returns mutable access to the bias for a variable, inserting zero when absent.
    fn index_mut(&mut self, index: VarIdx) -> &mut Self::Output {
        let pos = self.biases.find(index).unwrap_or_else(|l| l);
        if pos == self.len() {
            self.push_back(index, Bias::default());
        } else if self.biases[pos].idx != index {
            self.insert(pos, index, Bias::default());
        }
        &mut self.biases[pos].bias
    }
}

impl PartialEq for Linear {
    /// Compares the explicit sparse storage directly.
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.biases == other.biases
    }
}

impl Neg for Linear {
    type Output = Self;

    /// Negates every stored bias.
    fn neg(self) -> Self::Output {
        Self {
            biases: self.biases.iter().map(|(idx, b)| (idx, -b)).collect(),
        }
    }
}
