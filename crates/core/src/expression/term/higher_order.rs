use std::ops::{AddAssign, Index, IndexMut, Mul, MulAssign, Neg};

use lunamodel_types::{Bias, DEFAULT_BIAS, VarIdx};
use std::collections::HashMap;

use crate::traits::Editable;

static SEP: &str = "-";

/// Sparse storage for higher-order expression contributions.
///
/// Contributions are keyed by a canonicalized string representation of the
/// participating variable indices. This is less specialized than the linear and
/// quadratic storage, but it keeps arbitrary-degree terms straightforward to
/// insert, combine, and serialize.
#[derive(Default, Debug, Clone)]
pub struct HigherOrder {
    entries: HashMap<String, Bias>,
}
impl Editable for HigherOrder {}

impl HigherOrder {
    /// Creates a higher-order storage with a pre-allocated hash map capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            entries: HashMap::with_capacity(capacity),
        }
    }

    /// Returns the number of explicitly stored contributions.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns `true` if all stored contributions sum to zero.
    pub fn is_zero(&self) -> bool {
        self.iter().map(|(_, b)| b).sum::<Bias>() == Bias::default()
    }

    /// Returns `true` if no effective higher-order contribution is present.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty() || self.entries.values().sum::<Bias>() == Bias::default()
    }

    /// Iterates over the canonical contribution keys and their biases.
    pub fn iter(&self) -> impl Iterator<Item = (&String, Bias)> {
        self.entries.iter().map(|(k, b)| (k, *b))
    }

    /// Iterates mutably over the canonical contribution keys and their biases.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&String, &mut Bias)> {
        self.entries.iter_mut()
    }

    /// Iterates over decoded variable tuples and their biases.
    pub fn iter_contrib(&self) -> impl Iterator<Item = (Vec<VarIdx>, Bias)> {
        self.entries.iter().map(|(k, b)| (contribs(k), *b))
    }

    /// Removes explicitly stored zero contributions.
    pub fn clean(&mut self) {
        self.entries.retain(|_, b| *b != Bias::default());
    }

    /// Returns the maximum contribution arity.
    pub fn degree(&self) -> usize {
        self.iter_contrib().map(|(k, _)| k.len()).max().unwrap()
    }
}

impl MulAssign<Bias> for HigherOrder {
    /// Scales all stored higher-order biases.
    fn mul_assign(&mut self, rhs: Bias) {
        if rhs == Bias::default() {
            *self = Self::default();
            return;
        }
        for (_, bias) in self.iter_mut() {
            *bias *= rhs;
        }
    }
}

impl Mul<Bias> for HigherOrder {
    type Output = Self;

    /// Returns a scaled copy of the higher-order storage.
    fn mul(mut self, rhs: Bias) -> Self::Output {
        self *= rhs;
        self
    }
}

impl Index<&[VarIdx]> for HigherOrder {
    type Output = Bias;

    /// Looks up a contribution by variable tuple, defaulting to zero when absent.
    fn index(&self, index: &[VarIdx]) -> &Self::Output {
        &self[&key(index.to_vec())]
    }
}

impl IndexMut<&[VarIdx]> for HigherOrder {
    /// Returns mutable access to a contribution by variable tuple.
    fn index_mut(&mut self, index: &[VarIdx]) -> &mut Self::Output {
        &mut self[&key(index.to_vec())]
    }
}

impl Index<&String> for HigherOrder {
    type Output = Bias;

    /// Looks up a contribution by canonical string key.
    fn index(&self, index: &String) -> &Self::Output {
        self.entries.get(index).unwrap_or_else(|| &DEFAULT_BIAS)
    }
}

impl IndexMut<&String> for HigherOrder {
    /// Returns mutable access to a contribution by canonical string key.
    fn index_mut(&mut self, index: &String) -> &mut Self::Output {
        if !self.entries.contains_key(index) {
            self.entries.insert(index.clone(), Bias::default());
        }
        self.entries.get_mut(index).unwrap()
    }
}

impl Neg for HigherOrder {
    type Output = Self;

    /// Negates every stored higher-order bias.
    fn neg(self) -> Self::Output {
        Self {
            entries: self.entries.into_iter().map(|(k, b)| (k, -b)).collect(),
        }
    }
}

impl PartialEq for HigherOrder {
    /// Compares storages while treating implicit and explicit zeros equally.
    fn eq(&self, other: &Self) -> bool {
        let mut all: Vec<_> = self.entries.keys().collect();
        all.append(&mut other.entries.keys().collect());
        for &k in all.iter() {
            if self[k] != other[k] {
                return false;
            }
        }
        true
    }
}

/// Decodes the canonical string key back into variable indices.
fn contribs(str: &str) -> Vec<VarIdx> {
    str.split(SEP)
        .map(|s| s.parse::<VarIdx>().unwrap())
        .collect()
}

/// Canonicalizes a variable tuple into the internal key representation.
fn key(mut indices: Vec<VarIdx>) -> String {
    indices.sort();
    indices
        .into_iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join(SEP)
}

impl AddAssign<&HigherOrder> for HigherOrder {
    /// Adds all contributions from another higher-order storage.
    fn add_assign(&mut self, rhs: &HigherOrder) {
        for (contrib, bias) in rhs.iter() {
            self[contrib] += bias;
        }
    }
}

impl AddAssign<HigherOrder> for HigherOrder {
    /// Adds all contributions from another higher-order storage.
    fn add_assign(&mut self, rhs: HigherOrder) {
        self.add_assign(&rhs);
    }
}

impl AddAssign<(Vec<u32>, Bias)> for HigherOrder {
    /// Adds a single contribution from an owned variable tuple.
    fn add_assign(&mut self, rhs: (Vec<u32>, Bias)) {
        self[rhs.0.as_slice()] += rhs.1
    }
}

impl AddAssign<(&[u32], Bias)> for HigherOrder {
    /// Adds a single contribution from a borrowed variable tuple.
    fn add_assign(&mut self, rhs: (&[u32], Bias)) {
        self[rhs.0] += rhs.1
    }
}
