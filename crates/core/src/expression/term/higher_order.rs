//! Sparse storage for higher-order expression terms.

use std::ops::{AddAssign, Index, IndexMut, Mul, MulAssign, Neg};

use lunamodel_types::{Bias, DEFAULT_BIAS, VarIdx};
use smallvec::SmallVec;
use std::collections::HashMap;

use crate::traits::Editable;

/// Canonical (sorted) variable tuple of a higher-order contribution.
///
/// Up to four indices are stored inline, so typical HUBO terms need no heap
/// allocation for their key.
type Key = SmallVec<[VarIdx; 4]>;

/// Sparse storage for higher-order expression contributions.
///
/// Contributions are keyed by the sorted tuple of participating variable
/// indices. This is less specialized than the linear and quadratic storage,
/// but it keeps arbitrary-degree terms straightforward to insert, combine, and
/// serialize.
#[derive(Default, Debug, Clone)]
pub struct HigherOrder {
    entries: HashMap<Key, Bias>,
}
impl Editable for HigherOrder {}

impl HigherOrder {
    /// Creates a higher-order storage with a pre-allocated hash map capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            entries: HashMap::with_capacity(capacity),
        }
    }

    /// Returns the number of non-zero contributions.
    ///
    /// This counts exactly what [`iter`](Self::iter) and
    /// [`iter_contrib`](Self::iter_contrib) yield; contributions that cancelled
    /// out are still stored but are not counted.
    pub fn len(&self) -> usize {
        self.iter().count()
    }

    /// Returns `true` if all stored contributions sum to zero.
    pub fn is_zero(&self) -> bool {
        self.iter().map(|(_, b)| b).all(|b| b == Bias::default())
    }

    /// Returns `true` if no effective higher-order contribution is present.
    ///
    /// This agrees with [`len`](Self::len) and [`iter`](Self::iter).
    pub fn is_empty(&self) -> bool {
        self.iter().next().is_none()
    }

    /// Iterates over the sorted variable tuples and their biases.
    pub fn iter(&self) -> impl Iterator<Item = (&[VarIdx], Bias)> {
        self.entries
            .iter()
            .filter_map(|(k, b)| match *b != Bias::default() {
                true => Some((k.as_slice(), *b)),
                false => None,
            })
    }

    /// Iterates mutably over the sorted variable tuples and their biases.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&[VarIdx], &mut Bias)> {
        self.entries
            .iter_mut()
            .filter_map(|(k, b)| match *b != Bias::default() {
                true => Some((k.as_slice(), b)),
                false => None,
            })
    }

    /// Iterates over owned variable tuples and their biases.
    ///
    /// Prefer [`iter`](Self::iter) when the tuple does not need to be modified.
    pub fn iter_contrib(&self) -> impl Iterator<Item = (Vec<VarIdx>, Bias)> {
        self.iter().map(|(k, b)| (k.to_vec(), b))
    }

    /// Removes explicitly stored zero contributions.
    pub fn clean(&mut self) {
        self.entries.retain(|_, b| *b != Bias::default());
    }

    /// Returns the maximum contribution arity.
    pub fn degree(&self) -> usize {
        self.iter().map(|(k, _)| k.len()).max().unwrap()
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
        let found = match index.is_sorted() {
            true => self.entries.get(index),
            false => self.entries.get(key(index).as_slice()),
        };
        found.unwrap_or(&DEFAULT_BIAS)
    }
}

impl IndexMut<&[VarIdx]> for HigherOrder {
    /// Returns mutable access to a contribution by variable tuple.
    fn index_mut(&mut self, index: &[VarIdx]) -> &mut Self::Output {
        self.entries.entry(key(index)).or_default()
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
        self.entries.iter().all(|(k, b)| *b == other[k.as_slice()])
            && other.entries.iter().all(|(k, b)| *b == self[k.as_slice()])
    }
}

/// Canonicalizes a variable tuple into the internal key representation.
fn key(indices: &[VarIdx]) -> Key {
    let mut key = Key::from_slice(indices);
    key.sort_unstable();
    key
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
