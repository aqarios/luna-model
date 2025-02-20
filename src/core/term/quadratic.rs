use std::{
    iter::Enumerate,
    ops::{Index, IndexMut, MulAssign},
    slice::Iter,
};

use crate::core::expression::BiasConstraints;

use super::types::OneVarTerm;

#[derive(Debug, Clone)]
pub struct Quadratic<Index, Bias> {
    adj: Vec<Vec<OneVarTerm<Index, Bias>>>,
}

impl<Index, Bias> Quadratic<Index, Bias>
where
    Bias: Clone,
    OneVarTerm<Index, Bias>: Clone,
{
    pub fn new(num_variables: usize) -> Self {
        let adj = vec![Vec::new(); num_variables];
        Self { adj }
    }

    pub fn resize(&mut self, n: usize) {
        self.adj.resize(n, Vec::new());
    }

    pub fn len(&self) -> usize {
        self.adj.len()
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut Vec<OneVarTerm<Index, Bias>>> {
        self.adj.get_mut(idx)
    }

    pub fn iter(&self) -> Enumerate<Iter<Vec<OneVarTerm<Index, Bias>>>> {
        self.adj.iter().enumerate()
    }

    // pub fn iter_mut(&mut self) -> IterMut<(Index, Index, Bias)> {
    //     unimplemented!()
    // }
}

// impl<Index, Bias> Iterator for Quadratic<Index, Bias> {
//     type Item = (Index, Index, Bias);
//
//     fn next(&mut self) -> Option<Self::Item> {
//         todo!()
//     }
// }
//
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
