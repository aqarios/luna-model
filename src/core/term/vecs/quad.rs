use std::{
    cmp::Ordering,
    marker::PhantomData,
    ops::{Add, AddAssign},
};

use crate::core::Vtype;

pub type SizeType = usize;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
struct OneVarTerm<Index, Bias> {
    index: Index,
    bias: Bias,
}

impl<Index, Bias> OneVarTerm<Index, Bias>
where
    Bias: Default,
{
    fn new(index: Index, bias: Bias) -> Self {
        Self { index, bias }
    }

    fn new_default(index: Index) -> Self {
        Self {
            index,
            bias: Bias::default(),
        }
    }
}

pub struct QuadraticModelBase<Bias, Index> {
    linear_biases: Vec<Bias>,
    // adj: Vec<Vec<(Index, Bias)>>, // Adjacency list
    // Vec already heap allocated, no need for Box...
    adj: Option<Vec<Vec<OneVarTerm<Index, Bias>>>>,
    // Maybe even:
    // adj: Option<HashMap<Index, Vec<OneVarTerm<Index, Bias>>>>,
    offset: Bias,
    phantom_index: PhantomData<Index>,
}

impl<Bias, Index> QuadraticModelBase<Bias, Index>
where
    Bias: Copy + Default + AddAssign + Add<Output = Bias> + PartialEq,
    Index: Copy + Into<SizeType> + Default + PartialOrd + From<SizeType> + Ord,
{
    pub fn default() -> Self {
        Self {
            linear_biases: Vec::new(),
            adj: None,
            offset: Bias::default(),
            phantom_index: PhantomData,
        }
    }

    pub fn add_offset(&mut self, bias: Bias) {
        self.offset += bias
    }

    pub fn add_linear(&mut self, v: Index, bias: Bias) {
        // let v_idx = v.try_into().ok().expect("Failed to convert u to usize");
        let v_idx = v.into();
        assert!(
            v_idx >= 0 && v_idx < self.num_variables(),
            "v is out of range"
        );
        self.linear_biases[v_idx] += bias;
    }

    pub fn add_quadratic(&mut self, u: Index, v: Index, bias: Bias) {
        // let u_idx = u.try_into().ok().expect("Failed to convert u to usize");
        // let v_idx = v.try_into().ok().expect("Failed to convert v to usize");
        let u_idx = u.into();
        let v_idx = v.into();

        assert!(
            u_idx >= 0 && u_idx < self.num_variables(),
            "u is out of range"
        );
        assert!(
            v_idx >= 0 && v_idx < self.num_variables(),
            "u is out of range"
        );
        self.enforce_adj();

        if u_idx == v_idx {
            match self.vartype(u) {
                Vtype::Binary => {
                    // 1*1 == 1 and 0*0 == 0 so this is linear
                    self.linear_biases[u_idx] += bias;
                }
                Vtype::Spin => {
                    // -1*-1 == +1*+1 == 1 so this is constant offset
                    self.offset += bias;
                }
                _ => {
                    // self-loop
                    // dereferencing is perfectly fine here, zero-cost at runtime
                    // only affects access in compile time, does not introduce any extra copy or
                    // allocation.
                    *self.asymmetric_quadratic_ref(u, u) += bias;
                }
            }
        } else {
            *self.asymmetric_quadratic_ref(u, v) += bias;
            *self.asymmetric_quadratic_ref(v, u) += bias;
        }
    }

    pub fn add_quadratic_back(&mut self, u: Index, v: Index, bias: Bias) {
        // let u_idx = u.try_into().ok().expect("Failed to convert u to usize");
        // let v_idx = v.try_into().ok().expect("Failed to convert v to usize");
        let u_idx = u.into();
        let v_idx = v.into();

        assert!(
            u_idx >= 0 && u_idx < self.num_variables(),
            "u is out of range"
        );
        assert!(
            v_idx >= 0 && v_idx < self.num_variables(),
            "u is out of range"
        );
        self.enforce_adj();

        // Safe unwrap since we know it exists. due to the enforce_adj call.
        let adj = self.adj.as_ref().unwrap();

        // println!("u = {}", u_idx);
        // println!("v = {}", v_idx);
        // println!("adj[v_idx].is_empty() = {}", adj[v_idx].is_empty());
        // println!(
        //     "adj[v_idx].last().unwrap().index <= u = {}",
        //     adj[v_idx].last().unwrap().index <= u
        // );

        // println!("u_idx = {} & v_idx = {}", u_idx, v_idx);
        assert!(
            adj[v_idx].is_empty() || adj[v_idx].last().unwrap().index <= u,
            "Index out of order: last index > u"
        );
        assert!(
            adj[u_idx].is_empty() || adj[u_idx].last().unwrap().index <= v,
            "Index out of order: last index > v"
        );

        if u_idx == v_idx {
            match self.vartype(u) {
                Vtype::Binary => {
                    // 1*1 == 1 and 0*0 == 0 so this is linear
                    // self.add_linear(u, bias);
                    self.linear_biases[u_idx] += bias;
                }
                Vtype::Spin => {
                    // -1*-1 == +1*+1 == 1 so this is a constant offset
                    self.offset += bias;
                }
                _ => {
                    // self-loop
                    self.adj.as_mut().unwrap()[u_idx].push(OneVarTerm::new(v, bias));
                }
            }
        } else {
            let adj = self.adj.as_mut().unwrap();
            adj[u_idx].push(OneVarTerm::new(v, bias));
            adj[v_idx].push(OneVarTerm::new(u, bias));
        }
    }

    /// Assumes adj exists!
    /// Creates the bias if it doesn't already exist
    fn asymmetric_quadratic_ref(&mut self, u: Index, v: Index) -> &mut Bias {
        // let u_idx = u.try_into().ok().expect("Failed to convert u to usize");
        // let v_idx = v.try_into().ok().expect("Failed to convert v to usize");
        let u_idx = u.into();
        let v_idx = v.into();

        assert!(
            u_idx >= 0 && u_idx < self.num_variables(),
            "u is out of range"
        );
        assert!(
            v_idx >= 0 && v_idx < self.num_variables(),
            "u is out of range"
        );
        assert!(self.has_adj(), "adj is not initialized");

        let neighborhood: &mut Vec<OneVarTerm<Index, Bias>> = self
            .adj
            .as_mut()
            .and_then(|adj| adj.get_mut(u_idx))
            .expect("neighborhood should exist for the given index");

        // Find the position where v should be inserted
        let pos = neighborhood
            .binary_search_by(|term| term.index.partial_cmp(&v).unwrap_or(Ordering::Equal))
            .unwrap_or_else(|insert_pos| insert_pos);

        if pos == neighborhood.len() || neighborhood[pos].index != v {
            neighborhood.insert(pos, OneVarTerm::new_default(v));
        }

        &mut neighborhood[pos].bias
    }

    pub fn add_quadratic_from_dense(&mut self, dense: &[Bias], num_variables: Index) {
        let nvars = num_variables.into();

        // assert!(0 <= nvars, "no variables");
        assert!(
            self.num_variables() <= nvars,
            "more variables than in model"
        );
        self.enforce_adj();

        if self.is_linear() {
            for u in 0..nvars {
                // diagonal
                self.add_quadratic_back(u.into(), u.into(), dense[u * (nvars + 1)]);

                // off-diagonal
                for v in (u + 1)..nvars {
                    let qbias = dense[u * nvars + v] + dense[v * nvars + u];

                    if qbias != Bias::default() {
                        self.add_quadratic_back(u.into(), v.into(), qbias);
                    }
                }
            }
        } else {
            // we cannot rely on the ordering
            for u in 0..nvars {
                // diagonal
                self.add_quadratic(u.into(), u.into(), dense[u * (nvars + 1)]);

                // off-diagonal
                for v in (u + 1)..nvars {
                    let qbias = dense[u * nvars + v] + dense[v * nvars + u];

                    if qbias != Bias::default() {
                        self.add_quadratic(u.into(), v.into(), qbias);
                    }
                }
            }
        }
    }

    pub fn is_linear(&self) -> bool {
        let adj = self.adj.as_ref().unwrap();
        if self.has_adj() {
            for n in adj.iter() {
                if !n.is_empty() {
                    return false;
                }
            }
        }
        return true;
    }

    pub fn num_variables(&self) -> SizeType {
        self.linear_biases.len()
    }

    /// Increase the size of the model by one. Returns the index of the new variable.
    pub fn add_variable(&mut self) -> Index {
        self.add_variables(1.into())
    }

    /// Increase the size of the model by `n`. Returns the index of the first variable added.
    pub fn add_variables(&mut self, n: Index) -> Index {
        let size = self.num_variables();
        self.linear_biases.resize(size + n.into(), Bias::default());

        if self.has_adj() {
            let adj = self.adj.as_mut().unwrap();
            adj.resize(size + n.into(), Vec::new());
        }

        size.into()
    }

    pub fn resize(&mut self, n: Index) {
        if self.has_adj() {
            if n.into() < self.num_variables() {
                let adj = self.adj.as_mut().unwrap();
                for neighborhood in adj {
                    if let Ok(pos) = neighborhood.binary_search_by(|term| term.index.cmp(&n)) {
                        neighborhood.truncate(pos);
                    }
                }
            }
            self.adj.as_mut().unwrap().resize(n.into(), Vec::new());
        }

        self.linear_biases.resize(n.into(), Bias::default());

        assert!(!self.has_adj() || self.linear_biases.len() == self.adj.as_ref().unwrap().len());
    }

    fn enforce_adj(&mut self) {
        if !self.has_adj() {
            self.adj = Some(vec![Vec::new(); self.num_variables()]);
        }
    }

    #[inline]
    fn has_adj(&self) -> bool {
        self.adj.is_some()
    }

    fn vartype(&self, _v: Index) -> Vtype {
        // todo: implement actual
        Vtype::Binary
    }
}
