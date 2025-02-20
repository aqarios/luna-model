use std::cell::{Ref, RefCell};
use std::cmp::Ordering;
use std::rc::Rc;

use crate::core::exceptions::VariablesFromDifferentEnvsError;
use crate::core::operations::{AddAssignToExpression, AddToExpression, MulToExpression};
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
    pub higher_order: Option<HigherOrder<Index, Bias>>,
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

impl<Index, Bias> MulToExpression<Index, Bias, Bias> for &Expression<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    type Output = Expression<Index, Bias>;

    fn mul(self, rhs: Bias) -> Self::Output {
        let mut out = Expression::new_from(&self);
        out.offset *= rhs;
        out.linear *= rhs;
        if out.has_quadratic() {
            *out.quadratic.as_mut().unwrap() *= rhs;
        }
        if out.has_higher_order() {
            *out.higher_order.as_mut().unwrap() *= rhs;
        }
        out
    }
}

impl<Index, Bias> AddAssignToExpression<Index, Bias, Bias> for Expression<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    type Output = ();

    fn add_assign(&mut self, rhs: Bias) -> Self::Output {
        self.add_offset(rhs)
    }
}

impl<Index, Bias> AddAssignToExpression<Index, Bias, &VarRef<Index>> for Expression<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    type Output = Result<(), VariablesFromDifferentEnvsError>;

    fn add_assign(&mut self, rhs: &VarRef<Index>) -> Self::Output {
        if self.env.borrow().id != rhs.env.borrow().id {
            Err(VariablesFromDifferentEnvsError)
        } else {
            Ok(self.add_linear(rhs.id, Bias::one()))
        }
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

impl<Index, Bias> AddToExpression<Index, Bias, Ref<'_, Expression<Index, Bias>>>
    for &Expression<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    type Output = Result<Expression<Index, Bias>, VariablesFromDifferentEnvsError>;
    fn add(self, rhs: Ref<'_, Expression<Index, Bias>>) -> Self::Output {
        if self.env.borrow().id != rhs.env.borrow().id {
            Err(VariablesFromDifferentEnvsError)
        } else {
            let mut out = Expression::new_from(&self);
            // We know that both expressions have the same environment
            // so we just need to check if the sizes of the two expression matches, i.e.
            // if both expressions have the same number of variables.
            // If rhs has more variables than self, we need to resize the out to
            // allow the other variables to be added safely.
            if out.num_variables() < rhs.num_variables() {
                out.resize(rhs.num_variables().into());
            }
            // Now we can perform all additions safely.
            out.add_offset(rhs.offset);
            out.add_linear_from(&rhs.linear);

            if rhs.quadratic.is_some() {
                out.add_quadratic_from(rhs.quadratic.as_ref().unwrap());
            }
            if rhs.higher_order.is_some() {
                out.add_higher_order_from(rhs.higher_order.as_ref().unwrap());
            }
            Ok(out)
        }
    }
}

impl<Index, Bias> AddAssignToExpression<Index, Bias, Ref<'_, Expression<Index, Bias>>>
    for Expression<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    type Output = Result<(), VariablesFromDifferentEnvsError>;
    fn add_assign(&mut self, rhs: Ref<'_, Expression<Index, Bias>>) -> Self::Output {
        if self.env.borrow().id != rhs.env.borrow().id {
            Err(VariablesFromDifferentEnvsError)
        } else {
            let mut out = Expression::new_from(&self);
            // We know that both expressions have the same environment
            // so we just need to check if the sizes of the two expression matches, i.e.
            // if both expressions have the same number of variables.
            // If rhs has more variables than self, we need to resize the out to
            // allow the other variables to be added safely.
            if out.num_variables() < rhs.num_variables() {
                out.resize(rhs.num_variables().into());
            }
            // Now we can perform all additions safely.
            out.add_offset(rhs.offset);
            out.add_linear_from(&rhs.linear);

            if rhs.quadratic.is_some() {
                out.add_quadratic_from(rhs.quadratic.as_ref().unwrap());
            }
            if rhs.higher_order.is_some() {
                out.add_higher_order_from(rhs.higher_order.as_ref().unwrap());
            }
            Ok(())
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
            let mut out = Expression::new_from(&self);
            // We know that both expressions have the same environment
            // so we just need to check if the sizes of the two expression matches, i.e.
            // if both expressions have the same number of variables.
            // If rhs has more variables than self, we need to resize the out to
            // allow the other variables to be added safely.
            if out.num_variables() < rhs.num_variables() {
                out.resize(rhs.num_variables().into());
            }
            // Now we can perform all additions safely.
            out.add_offset(rhs.offset);
            out.add_linear_from(&rhs.linear);

            if rhs.quadratic.is_some() {
                out.add_quadratic_from(rhs.quadratic.as_ref().unwrap());
            }
            if rhs.higher_order.is_some() {
                out.add_higher_order_from(rhs.higher_order.as_ref().unwrap());
            }
            Ok(out)
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
            higher_order: None,
        }
    }

    fn new_from(other: &Self) -> Self {
        Self {
            env: other.env.clone(),
            offset: other.offset,
            linear: other.linear.clone(),
            quadratic: other.quadratic.clone(),
            higher_order: other.higher_order.clone(),
        }
    }

    fn new_linear_from_weighted_variable(
        env: Rc<RefCell<Environment<Index>>>,
        var: Index,
        bias: Bias,
    ) -> Self {
        Self {
            env,
            offset: Bias::default(),
            linear: Linear::new_from_weighted_variable(var.into(), bias),
            quadratic: None,
            higher_order: None,
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
            higher_order: None,
        }
    }

    fn new_linear(env: Rc<RefCell<Environment<Index>>>, linear_biases: &Vec<Bias>) -> Self {
        Self {
            env,
            offset: Bias::default(),
            linear: Linear::from(linear_biases),
            quadratic: None,
            higher_order: None,
        }
    }
    fn new_quadratic_from_variables(
        env: Rc<RefCell<Environment<Index>>>,
        u: Index,
        v: Index,
        bias: Bias,
    ) -> Self {
        let mut out = Self {
            env,
            offset: Bias::default(),
            linear: Linear::default(),
            quadratic: None,
            higher_order: None,
        };
        out.add_quadratic(u, v, bias);
        out
    }

    fn add_variable(&mut self) -> Index {
        self.add_variables(Index::one())
    }

    fn add_variables(&mut self, n: Index) -> Index {
        let size = self.num_variables();
        // If no variable is in the current expression yet
        if n == Index::default() && size == 0 {
            // the index is 0.
            return self.add_variable();
        }

        // if the variable is equal to the size, we need to add just a single
        // variable entry to the expression.
        if n.into() == size {
            self.linear.resize(size + 1);
            if self.has_quadratic() {
                let adj = self.quadratic.as_mut().unwrap();
                adj.resize(size + 1);
            }
            return size.into();
        }

        // we need to check if the variable was already added once.
        // this needs to be optimized at some point.
        if n.into() < size {
            // The variable is already represented in the expression. Thus we don't
            // need to do anything.
            // Does this make sense? Maybe we need to move away from the tedious
            // dimod implementation...while keeping the same internal structures.
            // Must not affect performance.
            return size.into();
        }

        self.linear.resize(size + n.into());

        if self.has_quadratic() {
            let adj = self.quadratic.as_mut().unwrap();
            adj.resize(size + n.into());
        }

        // Higher order terms are an abstraction over a HashMap.
        // Thus, we don't need to do anything here, as it dynamically
        // resizes on insertion.
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

        // Again, higher order terms do not need to be resized, see `add_variables`

        assert!(
            !self.has_quadratic() || self.linear.len() == self.quadratic.as_ref().unwrap().len()
        );
    }

    fn vartype_(&self, v: Index) -> Vtype {
        self.env.borrow().get_vtype(v)
    }
}

impl<Index, Bias> ExpressionBase<Index, Bias> for Expression<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    fn add_offset(&mut self, bias: Bias) {
        self.offset += bias
    }

    fn offset(&self) -> Bias {
        self.offset
    }

    fn add_linear(&mut self, v: Index, bias: Bias) {
        let v_idx = v.into();

        // Instead of panic, if the variable is not in the current expression, we add
        // it
        self.add_variables(v);
        // Sanity check.
        assert!(v_idx < self.num_variables(), "v is out of range");
        self.linear[v_idx] += bias;
    }

    fn add_linear_from(&mut self, other: &Linear<Bias>) {
        for (idx, bias) in other.iter() {
            self.linear[idx] += *bias;
        }
    }

    fn linear(&self, v: Index) -> Bias {
        let v_idx = v.into();
        assert!(v_idx < self.num_variables(), "v is out of range");
        self.linear[v_idx]
    }

    fn quadratic(&self, u: Index, v: Index) -> Bias {
        let u_idx = u.into();
        let v_idx = v.into();
        assert!(u_idx < self.num_variables(), "u is out of range");
        assert!(v_idx < self.num_variables(), "v is out of range");

        // if u_idx == v_idx {
        //     todo!("special logic required??")
        // }

        let outer: Index;
        let inner: Index;
        if u_idx < v_idx {
            outer = u;
            inner = v;
        } else {
            outer = v;
            inner = u;
        }
        let neighborhood = &self.quadratic.as_ref().unwrap()[outer.into()];
        let pos = neighborhood
            .binary_search_by(|term| term.index.partial_cmp(&inner).unwrap_or(Ordering::Equal));
        match pos {
            Ok(p) => neighborhood[p].bias,
            Err(_) => Bias::default(),
        }
    }

    fn higher_order(&self, indices: &Vec<Index>) -> Bias {
        if self.has_higher_order() {
            let ho = self.higher_order.as_ref().unwrap();
            ho[indices]
        } else {
            Bias::default()
        }
    }

    fn add_quadratic(&mut self, u: Index, v: Index, bias: Bias) {
        let u_idx = u.into();
        let v_idx = v.into();

        // Add the variables dynamically, if not existing.
        // It is sufficient to add the larger variable, the smaller one is
        // automatically created.
        self.add_variables(u);
        self.add_variables(v);

        assert!(
            u_idx < self.num_variables(),
            "add_quadratic: u is out of range (is {})",
            u.to_string()
        );
        assert!(
            v_idx < self.num_variables(),
            "add_quadratic: v is out of range (is {})",
            v.to_string()
        );

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

    fn add_quadratic_from(&mut self, other: &Quadratic<Index, Bias>) {
        for (u, neighborhood) in other.iter() {
            for item in neighborhood.iter() {
                self.add_quadratic(u.into(), item.index, item.bias);
            }
        }
    }

    fn add_higher_order_direct(&mut self, index: &String, bias: Bias) {
        // We need to check that each variable is in the model.
        self.enforce_higher_order();
        let ho = self.higher_order.as_mut().unwrap();
        ho[index] += bias
    }

    fn add_higher_order(&mut self, indices: &Vec<Index>, bias: Bias) {
        // We need to check that each variable is in the model.
        let max_index = indices.iter().max().unwrap();
        self.add_variables(*max_index);
        self.enforce_higher_order();
        let ho = self.higher_order.as_mut().unwrap();
        ho[indices] += bias
    }

    fn add_higher_order_from(&mut self, other: &HigherOrder<Index, Bias>) {
        for (key, bias) in other.iter() {
            self.add_higher_order_direct(key, *bias);
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

    fn vartype(&self, v: Index) -> Vtype {
        self.env.borrow().get_vtype(v)
    }
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

    /// Create the higher order structure if it doesn't already exist.
    fn enforce_higher_order(&mut self) {
        if !self.has_higher_order() {
            self.higher_order = Some(HigherOrder::default())
        }
    }

    #[inline]
    /// Return true if the model's quadraticacency structure exists.
    fn has_quadratic(&self) -> bool {
        self.quadratic.is_some()
    }

    #[inline]
    /// Return true if the model's higher order structure exists.
    fn has_higher_order(&self) -> bool {
        self.higher_order.is_some()
    }
}
