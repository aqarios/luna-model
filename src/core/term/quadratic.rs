use super::types::{OneVarTerm, OneVarTermConstruction};
use crate::types::{Bias, VarIndex};
use std::{
    cmp::Ordering,
    iter::Enumerate,
    ops::{Index, IndexMut, MulAssign, Neg},
    slice::Iter,
};

#[derive(Debug, Clone)]
pub struct Quadratic {
    pub adj: Vec<Vec<OneVarTerm>>,
    default_bias: Bias,
}

impl Quadratic
where
    OneVarTerm: Clone,
{
    pub fn new(num_variables: usize) -> Self {
        let adj = vec![Vec::new(); num_variables];
        Self {
            adj,
            default_bias: Bias::default(),
        }
    }

    pub fn new_from(adj: Vec<Vec<OneVarTerm>>) -> Self {
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

    pub fn get_mut(&mut self, idx: VarIndex) -> Option<&mut Vec<OneVarTerm>> {
        self.adj.get_mut(<VarIndex as Into<usize>>::into(idx))
    }

    pub fn iter(&self) -> Enumerate<Iter<Vec<OneVarTerm>>> {
        self.adj.iter().enumerate()
    }

    pub fn iter_flat(&self) -> impl Iterator<Item = (VarIndex, VarIndex, Bias)> + '_ {
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
    ) -> impl Iterator<Item = ((usize, usize), VarIndex, VarIndex, Bias)> + '_ {
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

    pub fn cleanup(&mut self) {
        for neighborhood in self.adj.iter_mut() {
            neighborhood.retain(|t| t.bias != Bias::default());
        }
    }
}

impl MulAssign<Bias> for Quadratic {
    fn mul_assign(&mut self, rhs: Bias) {
        for neighborhood in self.adj.iter_mut() {
            for term in neighborhood.iter_mut() {
                term.bias *= rhs;
            }
        }
    }
}

// Iterator struct
pub struct QuadraticIter<'a> {
    inner: std::slice::Iter<'a, Vec<OneVarTerm>>,
}

impl<'a> IntoIterator for &'a Quadratic {
    type Item = &'a Vec<OneVarTerm>;
    type IntoIter = QuadraticIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        QuadraticIter {
            inner: self.adj.iter(),
        }
    }
}

impl<'a> Iterator for QuadraticIter<'a> {
    type Item = &'a Vec<OneVarTerm>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

// Mutable Iterator struct
pub struct QuadraticIterMut<'a> {
    inner: std::slice::IterMut<'a, Vec<OneVarTerm>>,
}

// Implement IntoIterator for &mut Quadratic
impl<'a> IntoIterator for &'a mut Quadratic {
    type Item = &'a mut Vec<OneVarTerm>;
    type IntoIter = QuadraticIterMut<'a>;

    fn into_iter(self) -> Self::IntoIter {
        QuadraticIterMut {
            inner: self.adj.iter_mut(),
        }
    }
}

// Implement Iterator for QuadraticIterMut
impl<'a> Iterator for QuadraticIterMut<'a> {
    type Item = &'a mut Vec<OneVarTerm>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl Index<usize> for Quadratic {
    type Output = Vec<OneVarTerm>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.adj[index]
    }
}

impl IndexMut<usize> for Quadratic {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.adj[index]
    }
}

impl Index<(usize, usize)> for Quadratic {
    type Output = Bias;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let mut outer = index.0;
        let mut inner = index.1;
        if outer > inner {
            outer = index.1;
            inner = index.0;
        }

        let neighborhood: &Vec<OneVarTerm> = &self.adj[outer];
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

impl Index<(VarIndex, VarIndex)> for Quadratic {
    type Output = Bias;

    fn index(&self, index: (VarIndex, VarIndex)) -> &Self::Output {
        &self[(
            <VarIndex as Into<usize>>::into(index.0),
            <VarIndex as Into<usize>>::into(index.1),
        )]
    }
}

impl IndexMut<(VarIndex, VarIndex)> for Quadratic {
    /// Assumes quadratic exists!
    /// Creates the bias if it doesn't already exist
    fn index_mut(&mut self, index: (VarIndex, VarIndex)) -> &mut Self::Output {
        let mut outer = index.0;
        let mut inner = index.1;
        if outer > inner {
            outer = index.1;
            inner = index.0;
        }

        let neighborhood: &mut Vec<OneVarTerm> = self
            .adj
            .get_mut::<usize>(outer.into())
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

impl PartialEq for Quadratic {
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

impl Quadratic {
    fn negate(&self) -> Self {
        Quadratic::new_from(
            self.adj
                .iter()
                .map(|neighborhood| {
                    neighborhood
                        .iter()
                        .map(|term| OneVarTerm::new(term.index, -term.bias))
                        .collect()
                })
                .collect(),
        )
    }
}

impl Neg for Quadratic {
    type Output = Quadratic;

    fn neg(self) -> Self::Output {
        self.negate()
    }
}

impl Neg for &Quadratic {
    type Output = Quadratic;

    fn neg(self) -> Self::Output {
        self.negate()
    }
}
