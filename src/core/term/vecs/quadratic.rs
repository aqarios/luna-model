// use std::{
//     cmp::Ordering,
//     fmt::Debug,
//     ops::{Add, AddAssign},
// };
//
// use crate::core::Vtype;
//
// pub type SizeType = usize;
//
// pub trait QuadraticBase<Index, Bias> {
//     fn add_offset(&mut self, bias: Bias);
//     fn add_linear(&mut self, v: Index, bias: Bias);
//     fn add_quadratic(&mut self, u: Index, v: Index, bias: Bias);
//     fn add_quadratic_back(&mut self, u: Index, v: Index, bias: Bias);
//     fn add_quadratic_from_dense(&mut self, dense: &[Bias], num_variables: Index);
//     fn resize(&mut self, n: Index);
//     fn num_variables(&self) -> SizeType;
//     fn is_linear(&self) -> bool;
// }
//
// #[derive(Clone)]
// pub struct OneVarTerm<Index: Clone, Bias: Copy + Clone + Default> {
//     index: Index,
//     bias: Bias,
// }
//
// impl<Index, Bias> OneVarTerm<Index, Bias>
// where
//     Index: Clone,
//     Bias: Clone + Copy + Default,
// {
//     fn new(index: Index, bias: Bias) -> Self {
//         Self { index, bias }
//     }
//
//     fn new_default(v: Index) -> Self {
//         Self {
//             index: v,
//             bias: Bias::default(),
//         }
//     }
// }
//
// pub struct QuadraticModel<Index: Clone, Bias: Copy + Clone + Default + AddAssign> {
//     linear_biases: Vec<Bias>,
//     // In dimod CPP this is a unique pointer
//     adj_ptr: Option<Box<Vec<Vec<OneVarTerm<Index, Bias>>>>>,
//     offset: Bias,
// }
//
// impl<Index, Bias> QuadraticBase<Index, Bias> for QuadraticModel<Index, Bias>
// where
//     Index: Clone + Into<SizeType> + Copy + PartialOrd + Default + TryFrom<SizeType>,
//     Bias: Copy + Clone + Default + AddAssign + Add<Output = Bias> + PartialOrd,
//     <Index as TryFrom<SizeType>>::Error: Debug, // for<'a> &'a mut Bias: AddAssign<Bias>,
// {
//     fn add_offset(&mut self, bias: Bias) {
//         self.offset += bias
//     }
//
//     fn add_linear(&mut self, v: Index, bias: Bias) {
//         assert!(v.into() < self.num_variables(), "v is out of range");
//         self.linear_biases[v.into()] += bias;
//     }
//
//     fn add_quadratic(&mut self, u: Index, v: Index, bias: Bias) {
//         // Maybe we can get rid of this by further restricting some stuff, but generally, I don't
//         // think that this is really possible. We never know what the user might do...
//         assert!(u.into() < self.num_variables(), "u is out of range");
//         assert!(v.into() < self.num_variables(), "v is out of range");
//         self.enforce_adj();
//
//         if u == v {
//             match self.vartype(u) {
//                 Vtype::Binary => {
//                     // 1*1 == 1 and 0*0 == 0 so this is linear
//                     self.linear_biases[u.into()] += bias;
//                 }
//                 Vtype::Spin => {
//                     // -1*-1 == +1*+1 == 1 so this is constant offset
//                     self.offset += bias;
//                 }
//                 _ => {
//                     // self-loop
//                     // dereferencing is perfectly fine here, zero-cost at runtime
//                     // only affects access in compile time, does not introduce any extra copy or
//                     // allocation.
//                     *self.asymmetric_quadratic_ref(u, u) += bias;
//                 }
//             }
//         }
//     }
//
//     fn add_quadratic_back(&mut self, u: Index, v: Index, bias: Bias) {
//         assert!(u.into() < self.num_variables(), "u is out of range");
//         assert!(v.into() < self.num_variables(), "v is out of range");
//         self.enforce_adj();
//
//         // check the condition for adding at the back
//         assert!(
//             self.adj_ptr.as_ref().unwrap()[v.into()].is_empty()
//                 || self.adj_ptr.as_ref().unwrap()[v.into()]
//                     .last()
//                     .unwrap()
//                     .index
//                     <= u,
//             "Index out of order: last index > u"
//         );
//         assert!(
//             self.adj_ptr.as_ref().unwrap()[u.into()].is_empty()
//                 || self.adj_ptr.as_ref().unwrap()[u.into()]
//                     .last()
//                     .unwrap()
//                     .index
//                     <= v,
//             "Index out of order: last index > v"
//         );
//
//         if u == v {
//             match self.vartype(u) {
//                 Vtype::Binary => {
//                     // 1*1 == 1 and 0*0 == 0 so this is linear
//                     self.add_linear(u, bias);
//                 }
//                 Vtype::Spin => {
//                     // -1*-1 == +1*+1 == 1 so this is a constant offset
//                     self.offset += bias;
//                 }
//                 _ => {
//                     // self-loop
//                     self.adj_ptr.as_mut().unwrap()[u.into()].push(OneVarTerm::new(v, bias));
//                 }
//             }
//         } else {
//             self.adj_ptr.as_mut().unwrap()[u.into()].push(OneVarTerm::new(v, bias));
//             self.adj_ptr.as_mut().unwrap()[v.into()].push(OneVarTerm::new(u, bias));
//         }
//     }
//
//     /// This code essentially is the old MatrixTranslator functionality. However, the code can be
//     /// heavily optimized with this new way. dense is a flat representation of the matrix passed
//     /// we can transform to this more efficiently then to the vec<vec<>> also the matrix structure
//     /// does not help much in terms of better aligned indexing due to the nature of the access
//     /// pattern required here.
//     fn add_quadratic_from_dense(&mut self, dense: &[Bias], num_variables: Index) {
//         // todo: Can we get rid of all the into by changing output of num_variables() to Index
//         // type?? Probably, would this make sense? Maybe. Definitly should try it out.
//         assert!(num_variables.into() <= self.num_variables());
//         self.enforce_adj();
//
//         // helper to reduce call repeates.
//         let numvars: usize = num_variables.into();
//
//         if self.is_linear() {
//             for i in 0..numvars {
//                 // diagonal
//                 // u has usize type, however, we know that we can downcast to the index,
//                 // as this is the original value and we need the .into() on the num_variables
//                 // only for iteration, as this is only defined on primitive types...
//                 // thus we can safely go from the usize back to the Index.
//                 // we need to force this however, as the compiler doesn't know that...
//                 let u: Index = Index::try_from(i).unwrap();
//                 self.add_quadratic_back(u, u, dense[i * (numvars + 1)]);
//
//                 // off-diagonal
//                 for j in (i + 1)..numvars {
//                     let v: Index = Index::try_from(j).unwrap();
//                     let qbias: Bias = dense[i * numvars + j] + dense[j * numvars + i];
//                     if qbias != Bias::default() {
//                         self.add_quadratic_back(u, v, qbias);
//                     }
//                 }
//             }
//         } else {
//             // we cannot rely on the ordering
//             for i in 0..numvars {
//                 let u: Index = Index::try_from(i).unwrap();
//                 // diagonal
//                 self.add_quadratic(u, u, dense[i * (numvars + 1)]);
//
//                 // off-diagonal
//                 for j in (i + 1)..numvars {
//                     let v: Index = Index::try_from(j).unwrap();
//                     let qbias: Bias = dense[i * numvars + j] + dense[j * numvars + i];
//
//                     if qbias != Bias::default() {
//                         self.add_quadratic(u, v, qbias);
//                     }
//                 }
//             }
//         }
//     }
//
//     fn resize(&mut self, n: Index) {}
//
//     fn num_variables(&self) -> SizeType {
//         self.linear_biases.len()
//     }
//
//     fn is_linear(&self) -> bool {
//         if self.has_adj() {
//             // can unwrap due to has_adj check.
//             for n in self.adj_ptr.as_ref().unwrap().iter() {
//                 if !n.is_empty() {
//                     return false;
//                 }
//             }
//         }
//         true
//     }
// }
//
// impl<Index, Bias> QuadraticModel<Index, Bias>
// where
//     Index: Clone + Into<SizeType> + Copy + PartialOrd + Default + TryFrom<SizeType>,
//     Bias: Copy + Clone + Default + AddAssign + Add<Output = Bias> + PartialOrd,
//     <Index as TryFrom<SizeType>>::Error: Debug,
// {
//     pub fn default() -> Self {
//         Self {
//             linear_biases: Vec::new(),
//             adj_ptr: None,
//             offset: Bias::default(),
//         }
//     }
//
//     /// Assumes adj exists!
//     /// Creates the bias if it doesn't already exist
//     fn asymmetric_quadratic_ref(&mut self, u: Index, v: Index) -> &mut Bias {
//         // In contrast to cpp we can force Index to be castable to a usize. see constraints on
//         // Index in where statement
//         assert!(u.into() < self.num_variables(), "u is out of range");
//         assert!(v.into() < self.num_variables(), "v is out of range");
//         assert!(self.has_adj(), "adj_ptr is not initialized");
//
//         let neighborhood: &mut Vec<OneVarTerm<Index, Bias>> = self
//             .adj_ptr
//             .as_mut()
//             .and_then(|adj| adj.get_mut(u.into()))
//             .expect("neighborhood should exist for the given index");
//
//         // Find the position where v should be inserted
//         let pos: usize = neighborhood
//             .binary_search_by(|term| term.index.partial_cmp(&v).unwrap_or(Ordering::Equal))
//             .unwrap_or_else(|e| e);
//
//         if pos == neighborhood.len() || neighborhood[pos].index != v {
//             // Insert a new OneVarTerm at the correct position with default bias (0)
//             neighborhood.insert(pos, OneVarTerm::new_default(v));
//         }
//
//         &mut neighborhood[pos].bias
//     }
//
//     fn enforce_adj(&mut self) {
//         if !self.has_adj() {
//             self.adj_ptr = Some(Box::new(vec![Vec::new(); self.num_variables()]))
//         }
//     }
//
//     #[inline]
//     fn has_adj(&self) -> bool {
//         self.adj_ptr.is_some()
//     }
//
//     #[inline]
//     fn vartype(&self, _v: Index) -> Vtype {
//         // todo: need the actual logic here...
//         Vtype::Binary
//     }
// }
