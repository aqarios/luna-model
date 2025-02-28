use std::{
    cmp::Ordering,
    iter::Enumerate,
    ops::{Index, IndexMut, MulAssign},
    slice::Iter,
};

use crate::core::expression::{BiasConstraints, IndexConstraints};

use super::types::{OneVarTerm, OneVarTermConstruction};

#[derive(Debug, Clone)]
pub struct Quadratic<Index, Bias> {
    adj: Vec<Vec<OneVarTerm<Index, Bias>>>,
    default_bias: Bias,
}

impl<Index, Bias> Quadratic<Index, Bias>
where
    Bias: BiasConstraints,
    OneVarTerm<Index, Bias>: Clone,
    Index: IndexConstraints,
{
    pub fn new(num_variables: usize) -> Self {
        let adj = vec![Vec::new(); num_variables];
        Self {
            adj,
            default_bias: Bias::default(),
        }
    }

    pub fn new_from(adj: Vec<Vec<OneVarTerm<Index, Bias>>>) -> Self {
        Self {
            adj,
            default_bias: Bias::default(),
        }
    }

    pub fn resize(&mut self, n: usize) {
        self.adj.resize(n, Vec::new());
    }

    pub fn len(&self) -> usize {
        self.adj.len()
    }

    pub fn is_empty(&self) -> bool {
        for neighborhood in self.adj.iter() {
            if !neighborhood.is_empty() {
                return false;
            }
        }
        true
    }

    pub fn get_mut(&mut self, idx: Index) -> Option<&mut Vec<OneVarTerm<Index, Bias>>> {
        self.adj.get_mut(idx.into())
    }

    pub fn iter(&self) -> Enumerate<Iter<Vec<OneVarTerm<Index, Bias>>>> {
        self.adj.iter().enumerate()
    }

    pub fn iter_flat(&self) -> impl Iterator<Item = (Index, Index, Bias)> + '_ {
        self.adj
            .iter()
            .enumerate()
            .flat_map(|(u_idx, neighborhood)| {
                neighborhood
                    .iter()
                    .map(move |term| (u_idx.into(), term.index, term.bias))
            })
    }

    pub fn iter_flat_positioned(
        &self,
    ) -> impl Iterator<Item = ((usize, usize), Index, Index, Bias)> + '_ {
        self.adj
            .iter()
            .enumerate()
            .flat_map(|(u_idx, neighborhood)| {
                neighborhood
                    .iter()
                    .enumerate()
                    .map(move |(v_idx, term)| ((u_idx, v_idx), u_idx.into(), term.index, term.bias))
            })
    }
}

impl<Index, Bias> MulAssign<Bias> for Quadratic<Index, Bias>
where
    Bias: BiasConstraints,
{
    fn mul_assign(&mut self, rhs: Bias) {
        for neighborhood in self.adj.iter_mut() {
            for term in neighborhood.iter_mut() {
                term.bias *= rhs;
            }
        }
    }
}

// Iterator struct
pub struct QuadraticIter<'a, Index, Bias> {
    inner: std::slice::Iter<'a, Vec<OneVarTerm<Index, Bias>>>,
}

impl<'a, Index, Bias> IntoIterator for &'a Quadratic<Index, Bias> {
    type Item = &'a Vec<OneVarTerm<Index, Bias>>;
    type IntoIter = QuadraticIter<'a, Index, Bias>;

    fn into_iter(self) -> Self::IntoIter {
        QuadraticIter {
            inner: self.adj.iter(),
        }
    }
}

impl<'a, Index, Bias> Iterator for QuadraticIter<'a, Index, Bias> {
    type Item = &'a Vec<OneVarTerm<Index, Bias>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

// Mutable Iterator struct
pub struct QuadraticIterMut<'a, Index, Bias> {
    inner: std::slice::IterMut<'a, Vec<OneVarTerm<Index, Bias>>>,
}

// Implement IntoIterator for &mut Quadratic
impl<'a, Index, Bias> IntoIterator for &'a mut Quadratic<Index, Bias> {
    type Item = &'a mut Vec<OneVarTerm<Index, Bias>>;
    type IntoIter = QuadraticIterMut<'a, Index, Bias>;

    fn into_iter(self) -> Self::IntoIter {
        QuadraticIterMut {
            inner: self.adj.iter_mut(),
        }
    }
}

// Implement Iterator for QuadraticIterMut
impl<'a, Index, Bias> Iterator for QuadraticIterMut<'a, Index, Bias> {
    type Item = &'a mut Vec<OneVarTerm<Index, Bias>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<Idx, Bias> Index<usize> for Quadratic<Idx, Bias> {
    type Output = Vec<OneVarTerm<Idx, Bias>>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.adj[index]
    }
}

impl<Idx, Bias> IndexMut<usize> for Quadratic<Idx, Bias> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.adj[index]
    }
}

impl<Idx, Bias> Index<(usize, usize)> for Quadratic<Idx, Bias>
where
    Idx: IndexConstraints,
    Bias: BiasConstraints,
{
    type Output = Bias;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let mut outer = index.0;
        let mut inner = index.1;
        if outer > inner {
            outer = index.1;
            inner = index.0;
        }

        let neighborhood: &Vec<OneVarTerm<Idx, Bias>> = &self.adj[outer];
        let pos = neighborhood.binary_search_by(|term| {
            term.index
                .partial_cmp(&inner.into())
                .unwrap_or(Ordering::Equal)
        });

        match pos {
            Ok(p) => &neighborhood[p].bias,
            Err(_) => &self.default_bias,
        }
    }
}

impl<Idx, Bias> Index<(Idx, Idx)> for Quadratic<Idx, Bias>
where
    Idx: IndexConstraints,
    Bias: BiasConstraints,
{
    type Output = Bias;

    fn index(&self, index: (Idx, Idx)) -> &Self::Output {
        &self[(index.0.into(), index.1.into())]
    }
}

impl<Idx, Bias> IndexMut<(Idx, Idx)> for Quadratic<Idx, Bias>
where
    Idx: IndexConstraints,
    Bias: BiasConstraints,
{
    /// Assumes quadratic exists!
    /// Creates the bias if it doesn't already exist
    fn index_mut(&mut self, index: (Idx, Idx)) -> &mut Self::Output {
        let mut outer = index.0;
        let mut inner = index.1;
        if outer > inner {
            outer = index.1;
            inner = index.0;
        }

        let neighborhood: &mut Vec<OneVarTerm<Idx, Bias>> = self
            .adj
            .get_mut(outer.into())
            .expect("neighborhood should exist for the given index");
        let pos = neighborhood
            .binary_search_by(|term| term.index.partial_cmp(&inner).unwrap_or(Ordering::Equal))
            .unwrap_or_else(|insert_pos| insert_pos);
        if pos == neighborhood.len() {
            neighborhood.push(OneVarTerm::new_default(inner))
        } else if neighborhood[pos].index != inner {
            neighborhood.insert(pos, OneVarTerm::new_default(inner));
        }

        &mut neighborhood[pos].bias
    }
}

impl<Index, Bias> PartialEq for Quadratic<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    fn eq(&self, other: &Self) -> bool {
        // This basic check is no gurantee for actual equality.
        // As if this is not equal it might be due to different representations,
        // e.g., in one expression the interaction between two variables can be explicitly
        // contained as 0.0, while in an other expression this interaction is not
        // represented directly. The value of the interaction is still 0.0.
        // Thus they are equal... This is not handled by the below trivial comparison.
        //
        // self.adj == other.adj
        if self.adj.len() != other.adj.len() {
            // Quick check if they have the same number of variables.
            return false;
        }
        for u_idx in 0..self.adj.len() {
            for v_idx in u_idx..self.adj.len() {
                // We iterate over the upper triangular matrix and check for each
                // possible combination if they are equal in both.
                let self_bias = self[(u_idx, v_idx)];
                let other_bias = other[(u_idx, v_idx)];
                if self_bias != other_bias {
                    return false;
                }
            }
        }

        true
    }
}
