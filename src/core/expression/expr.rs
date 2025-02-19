use std::cell::RefCell;
use std::cmp::Ordering;
use std::ops::Add;
use std::rc::Rc;

use crate::core::exceptions::VariablesFromDifferentEnvsError;
use crate::core::operations::AddToExpression;
use crate::core::term::types::{OneVarTerm, OneVarTermConstruction, SizeType};
use crate::core::term::{HigherOrder, Linear, Quadratic};
use crate::core::{Environment, VarRef, Vtype};

use super::base::{BiasConstraints, ExpressionBase, ExpressionBaseInternal, IndexConstraints};

pub struct Expression<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    pub env: Rc<RefCell<Environment<Index>>>,
    pub offset: Bias,
    pub linear: Linear<Bias>,
    pub quadratic: Option<Quadratic<Index, Bias>>,
    // pub higher_order: Option<HigherOrder<Index, Bias>>,
}

impl<Index, Bias> Expression<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
}

impl<Index, Bias> AddToExpression<Index, Bias, Bias> for &Expression<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    type Output = Expression<Index, Bias>;
    fn add(self, rhs: Bias) -> Self::Output {
        let mut out = Expression::new_from(&self);
        out.add_offset(rhs);
        out
    }
}

impl<Index, Bias> AddToExpression<Index, Bias, &VarRef<Index>> for &Expression<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    type Output = Result<Expression<Index, Bias>, VariablesFromDifferentEnvsError>;
    fn add(self, rhs: &VarRef<Index>) -> Self::Output {
        if self.env.borrow().id != rhs.env.borrow().id {
            Err(VariablesFromDifferentEnvsError)
        } else {
            let mut out = Expression::new_from(&self);
            out.add_linear(rhs.id, Bias::one());
            Ok(out)
        }
    }
}

impl<Index, Bias> AddToExpression<Index, Bias, &Expression<Index, Bias>>
    for &Expression<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    type Output = Result<Expression<Index, Bias>, VariablesFromDifferentEnvsError>;
    fn add(self, rhs: &Expression<Index, Bias>) -> Self::Output {
        if self.env.borrow().id != rhs.env.borrow().id {
            Err(VariablesFromDifferentEnvsError)
        } else {
            todo!()
            // let mut out = Expression::new_from(&self);
            // out.add_linear_from(rhs.linear);
            // out.add_quadratic_from(rhs.quadratic);
            // Ok(out)
        }
    }
}

impl<Index, Bias> ExpressionBaseInternal<Index, Bias> for Expression<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    fn new(env: Rc<RefCell<Environment<Index>>>) -> Self {
        Self {
            env,
            offset: Bias::default(),
            linear: Linear::default(),
            quadratic: None,
            // higher_order: None,
        }
    }

    fn new_from(other: &Self) -> Self {
        Self {
            env: other.env.clone(),
            offset: other.offset,
            linear: other.linear.clone(),
            quadratic: other.quadratic.clone(),
        }
    }

    fn new_from_weighted_variable(
        env: Rc<RefCell<Environment<Index>>>,
        var: Index,
        bias: Bias,
    ) -> Self {
        Self {
            env,
            offset: Bias::default(),
            linear: Linear::new_from_weighted_variable(var.into(), bias),
            quadratic: None,
        }
    }

    fn new_linear_from_variables(
        env: Rc<RefCell<Environment<Index>>>,
        lhs: Index,
        rhs: Index,
        bias: Bias,
    ) -> Self {
        Self {
            env,
            offset: Bias::default(),
            linear: Linear::new_from_variables(lhs.into(), rhs.into(), bias),
            quadratic: None,
        }
    }

    fn new_linear(env: Rc<RefCell<Environment<Index>>>, linear_biases: &Vec<Bias>) -> Self {
        Self {
            env,
            offset: Bias::default(),
            linear: Linear::from(linear_biases),
            quadratic: None,
            // higher_order: None,
        }
    }

    fn add_variable(&mut self) -> Index {
        self.add_variables(Index::one())
    }

    fn add_variables(&mut self, n: Index) -> Index {
        if n == Index::default() {
            // the index is 0.
            self.add_variable();
        }

        let size = self.num_variables();
        self.linear.resize(size + n.into());

        if self.has_quadratic() {
            let adj = self.quadratic.as_mut().unwrap();
            adj.resize(size + n.into());
        }

        size.into()
    }

    fn resize(&mut self, n: Index) {
        if self.has_quadratic() {
            if n.into() < self.num_variables() {
                let quadratic = self.quadratic.as_mut().unwrap();
                for neighborhood in quadratic {
                    if let Ok(pos) = neighborhood.binary_search_by(|term| term.index.cmp(&n)) {
                        neighborhood.truncate(pos);
                    }
                }
            }
            self.quadratic.as_mut().unwrap().resize(n.into());
        }

        self.linear.resize(n.into());

        assert!(
            !self.has_quadratic() || self.linear.len() == self.quadratic.as_ref().unwrap().len()
        );
    }

    // fn vartype_(&self, v: Index) -> Vtype {
    //     unimplemented!()
    // }
}

impl<Index, Bias> ExpressionBase<Index, Bias> for Expression<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    fn add_linear(&mut self, v: Index, bias: Bias) {
        let v_idx = v.into();

        // Instead of panic, if the variable is not in the current expression, we add
        // it
        self.add_variables(v);
        assert!(v_idx < self.num_variables(), "v is out of range");
        self.linear[v_idx] += bias;
    }

    fn add_offset(&mut self, bias: Bias) {
        self.offset += bias
    }

    fn linear(&self, v: Index) -> Bias {
        let v_idx = v.into();
        assert!(v_idx < self.num_variables(), "v is out of range");
        self.linear[v_idx]
    }

    fn add_quadratic(&mut self, u: Index, v: Index, bias: Bias) {
        let u_idx = u.into();
        let v_idx = v.into();

        assert!(u_idx < self.num_variables(), "u is out of range");
        assert!(v_idx < self.num_variables(), "v is out of range");
        self.enforce_quadratic();

        if u_idx == v_idx {
            match self.vartype(u) {
                Vtype::Binary => {
                    // 1*1 == 1 and 0*0 == 0 so this is linear
                    self.linear[u_idx] += bias;
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
            // We only store the upper right triangle matrix. Thus we need to check
            // which index is smaller. This index is used for the outer access.
            // The larger index is used for the smaller index's neighborhood.
            if u_idx < v_idx {
                *self.asymmetric_quadratic_ref(u, v) += bias;
            } else {
                *self.asymmetric_quadratic_ref(v, u) += bias;
            }
        }
    }

    fn add_quadratic_back(&mut self, u: Index, v: Index, bias: Bias) {
        let u_idx = u.into();
        let v_idx = v.into();

        assert!(u_idx < self.num_variables(), "u is out of range");
        assert!(v_idx < self.num_variables(), "u is out of range");
        self.enforce_quadratic();

        // Safe unwrap since we know it exists. due to the enforce_quadratic call.
        let quadratic = self.quadratic.as_ref().unwrap();

        assert!(
            quadratic[v_idx].is_empty() || quadratic[v_idx].last().unwrap().index <= u,
            "Index out of order: last index > u"
        );
        assert!(
            quadratic[u_idx].is_empty() || quadratic[u_idx].last().unwrap().index <= v,
            "Index out of order: last index > v"
        );

        if u_idx == v_idx {
            match self.vartype(u) {
                Vtype::Binary => {
                    // 1*1 == 1 and 0*0 == 0 so this is linear
                    // self.add_linear(u, bias);
                    self.linear[u_idx] += bias;
                }
                Vtype::Spin => {
                    // -1*-1 == +1*+1 == 1 so this is a constant offset
                    self.offset += bias;
                }
                _ => {
                    // self-loop
                    self.quadratic.as_mut().unwrap()[u_idx].push(OneVarTerm::new(v, bias));
                }
            }
        } else {
            let quadratic = self.quadratic.as_mut().unwrap();
            // We only store the upper right triangle matrix. Thus we need to check
            // which index is smaller. This index is used for the outer access.
            // The larger index is used for the smaller index's neighborhood.
            if u_idx < v_idx {
                quadratic[u_idx].push(OneVarTerm::new(v, bias));
            } else {
                quadratic[v_idx].push(OneVarTerm::new(u, bias));
            }
        }
    }

    fn add_quadratic_from_dense(&mut self, dense: &[Bias], num_variables: Index) {
        let nvars = num_variables.into();

        // assert!(0 <= nvars, "no variables");
        assert!(
            self.num_variables() <= nvars,
            "more variables than in model"
        );
        self.enforce_quadratic();

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

    fn is_linear(&self) -> bool {
        let quadratic = self.quadratic.as_ref().unwrap();
        if self.has_quadratic() {
            for n in quadratic {
                if !n.is_empty() {
                    return false;
                }
            }
        }
        return true;
    }

    fn num_variables(&self) -> SizeType {
        self.linear.len()
    }

    // fn vartype(&self, v: Index) -> Vtype {
    //     // self.env.as_ref().get()
    // }
}

impl<Index, Bias> Expression<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    /// Assumes quadratic exists!
    /// Creates the bias if it doesn't already exist
    fn asymmetric_quadratic_ref(&mut self, u: Index, v: Index) -> &mut Bias {
        // fn asymmetric_quadratic_ref(&mut self, u: Index, v: Index) -> QuadraticModelResult<&mut Bias> {
        let u_idx = u.into();
        let v_idx = v.into();

        assert!(u_idx < self.num_variables(), "u is out of range");
        assert!(v_idx < self.num_variables(), "u is out of range");
        assert!(self.has_quadratic(), "quadratic is not initialized");

        let neighborhood: &mut Vec<OneVarTerm<Index, Bias>> = self
            .quadratic
            .as_mut()
            .and_then(|quadratic| quadratic.get_mut(u_idx))
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

    /// Create the quadraticacency structure if it doesn't already exist.
    fn enforce_quadratic(&mut self) {
        if !self.has_quadratic() {
            self.quadratic = Some(Quadratic::new(self.num_variables()))
        }
    }

    #[inline]
    /// Return true if the model's quadraticacency structure exists.
    fn has_quadratic(&self) -> bool {
        self.quadratic.is_some()
    }
}
