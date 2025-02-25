use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

use hashbrown::HashMap;

use crate::core::term::types::{OneVarTerm, OneVarTermConstruction, SizeType};
use crate::core::term::{HigherOrder, Linear, Quadratic};
use crate::core::{Environment, Vtype};

use super::base::{
    BiasConstraints, ExpressionBase, ExpressionBaseAdd, ExpressionBaseAdjustment,
    ExpressionBaseCreation, ExpressionBaseMul, ExpressionBaseMulDirect, ExpressionBaseSet,
    ExpressionBaseTypes, IndexConstraints,
};
use super::errors::IndexOutOfOrderError;
use super::VariableOutOfRangeError;

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
    /// Mirror of the linear array that tracks which variables are already
    /// contained in the expression. 1 indicates already added 0 indicating floating.
    active: Vec<bool>,
    num_variables: SizeType,
}

impl<Index, Bias> ExpressionBaseTypes for Expression<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    type LinearType = Linear<Bias>;
    type QuadraticType = Quadratic<Index, Bias>;
    type HigherOrderType = HigherOrder<Index, Bias>;

    type HigherOrderKey = String;
}

impl<Index, Bias> ExpressionBaseCreation<Index, Bias> for Expression<Index, Bias>
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
            active: Vec::default(),
            num_variables: 0,
        }
    }
    fn new_linear_single(env: Rc<RefCell<Environment<Index>>>, v: Index, bias: Bias) -> Self {
        let linear = Linear::new_from_weighted_variable(v.into(), bias);
        // todo: make this it's own struct similar to linear.
        let mut active = Vec::new();
        active.resize(linear.len(), false);
        active[v.into()] = true;

        Self {
            env,
            offset: Bias::default(),
            linear,
            quadratic: None,
            higher_order: None,
            active,
            num_variables: 1,
        }
    }
    fn new_linear(env: Rc<RefCell<Environment<Index>>>, u: Index, v: Index, bias: Bias) -> Self {
        let linear = Linear::new_from_variables(u.into(), v.into(), bias);
        // todo: make this it's own struct similar to linear.
        let mut active = Vec::new();
        active.resize(linear.len(), false);
        active[u.into()] = true;
        active[v.into()] = true;

        Self {
            env,
            offset: Bias::default(),
            linear,
            quadratic: None,
            higher_order: None,
            active,
            num_variables: 2,
        }
    }
    fn new_quadratic(env: Rc<RefCell<Environment<Index>>>, u: Index, v: Index, bias: Bias) -> Self {
        let mut out = Self {
            env,
            offset: Bias::default(),
            linear: Linear::default(),
            quadratic: None,
            higher_order: None,
            active: Vec::default(),
            num_variables: 0,
        };
        out.add_quadratic(u, v, bias);
        out
    }
    fn new_from_other(other: &Self) -> Self {
        Self {
            env: other.env.clone(),
            offset: other.offset,
            linear: other.linear.clone(),
            quadratic: other.quadratic.clone(),
            higher_order: other.higher_order.clone(),
            active: other.active.clone(),
            num_variables: other.num_variables,
        }
    }
}

impl<Index, Bias> ExpressionBaseAdjustment<Index, Bias> for Expression<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    fn add_variable(&mut self, v: Index) -> SizeType {
        // First, we need to check if the variable is larger or equal to the current
        // size of the linear terms.
        let size = self.linear.len();
        if v.into() >= size {
            // We can simply resize the linear term and the active tracker.
            self.linear.resize(v.into() + 1);
            self.active.resize(v.into() + 1, false);
            // We also need to resize the quadratic term if this expression has one.
            if self.has_quadratic() {
                self.quadratic.as_mut().unwrap().resize(v.into() + 1);
            }
            // We do not need to resize the higher order term as in this implementation
            // it grows dynamically.
            // ...
            // Now we set the currently added variable as active.
            self.active[v.into()] = true;
            // And incresae the variable counter by one.
            self.num_variables += 1;
        } else {
            // v < size
            // We do not need to do any resizing.
            // And can directly check if the variable is active.
            let active = self.active[v.into()];
            // If the variable is already activated we need to do nothing.
            // This means the variable was already added at some point.
            // Otherwise, we need to activate it and increase the var counter by one.
            if !active {
                self.active[v.into()] = true;
                self.num_variables += 1;
            }
        }

        v.into()
    }

    fn add_variables(&mut self, vars: &Vec<Index>) {
        // We only need to call the add_variable for the largest index.
        // This will automatically allocate memory for all others.
        let max_index = vars.iter().max().unwrap();
        self.add_variable(*max_index);
        // Now we need to set each variable as active and increase the variable counter
        // if the variable has not been added before.
        for v in vars {
            let active = self.active[(*v).into()];
            if !active {
                self.active[(*v).into()] = true;
                self.num_variables += 1;
            }
        }
    }

    fn add_active_variables(&mut self, n: Index) {
        self.add_variable(n);
        // Maybe something more efficient?
        self.active = Vec::new();
        self.active.resize(n.into(), true);
    }

    fn resize(&mut self, n: Index) {
        if self.has_quadratic() {
            if n.into() < self.linear.len() {
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
}

impl<Index, Bias> ExpressionBase<Index, Bias> for Expression<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    fn offset(&self) -> Bias {
        self.offset
    }

    fn linear(&self, v: Index) -> Result<Bias, VariableOutOfRangeError> {
        let v_idx = self.check_and_get(v)?;
        Ok(self.linear[v_idx])
    }

    fn quadratic(&self, u: Index, v: Index) -> Result<Bias, VariableOutOfRangeError> {
        self.check_and_get(u)?;
        self.check_and_get(v)?;
        let mut bias = Bias::default();
        match self.has_quadratic() {
            true => {
                let mut outer = u;
                let mut inner = v;
                if u > v {
                    outer = v;
                    inner = u;
                }

                // TODO@benjamin: move the indexing to the quadratic term,
                // similar to how it's done for the higher order access
                // see `fn higher_order(&self, ...) -> ...` function.
                let neighborhood = &self.quadratic.as_ref().unwrap()[outer.into()];
                let pos = neighborhood.binary_search_by(|term| {
                    term.index.partial_cmp(&inner).unwrap_or(Ordering::Equal)
                });
                match pos {
                    Ok(p) => bias = neighborhood[p].bias,
                    Err(_) => (),
                }
            }
            false => (),
        }
        Ok(bias)
    }

    fn higher_order(&self, indices: &Vec<Index>) -> Result<Bias, VariableOutOfRangeError> {
        self.check_multi(indices)?;
        let res = match self.has_higher_order() {
            true => self.higher_order.as_ref().unwrap()[indices],
            false => Bias::default(),
        };
        Ok(res)
    }

    fn num_variables(&self) -> SizeType {
        self.num_variables
    }

    fn vartype(&self, v: Index) -> Vtype {
        self.env.borrow().get_vtype(v)
    }

    fn is_linear(&self) -> bool {
        let is_quadratic_empty = match self.has_quadratic() {
            true => self.quadratic.as_ref().unwrap().is_empty(),
            false => true,
        };
        let is_higher_order_empty = match self.has_higher_order() {
            true => self.higher_order.as_ref().unwrap().is_empty(),
            false => true,
        };
        is_quadratic_empty && is_higher_order_empty
    }
}

impl<Index, Bias> ExpressionBaseAdd<Index, Bias> for Expression<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    fn add_offset(&mut self, bias: Bias) {
        self.offset += bias
    }
    fn add_linear(&mut self, v: Index, bias: Bias) {
        self.add_variable(v);
        self.linear[v.into()] += bias;
    }

    fn add_quadratic(&mut self, u: Index, v: Index, bias: Bias) {
        self.add_variable(u);
        self.add_variable(v);
        let u_idx = u.into();
        let v_idx = v.into();
        self.enforce_quadratic();

        match (u_idx == v_idx, self.vartype(u)) {
            // -1*-1 == +1*+1 == 1 so this is constant offset
            (true, Vtype::Spin) => self.offset += bias,
            // 1*1 == 1 and 0*0 == 0 so this is linear
            (true, Vtype::Binary) => self.linear[u_idx] += bias,
            (_, _) => *self.asymmetric_quadratic_ref(u, v) += bias,
        }
    }
    fn add_higher_order(&mut self, vars: &Vec<Index>, bias: Bias) {
        self.add_variables(vars);
        self.enforce_higher_order();
        let contributions = self.reduce_higher_order_vars(vars);
        match contributions.len() {
            0 => self.add_offset(bias),
            1 => self.add_linear(contributions[0], bias),
            2 => self.add_quadratic(contributions[0], contributions[1], bias),
            _ => self.higher_order.as_mut().unwrap()[&contributions] += bias,
        }
    }

    fn add_higher_order_direct(&mut self, key: &Self::HigherOrderKey, bias: Bias) {
        self.enforce_higher_order();
        self.higher_order.as_mut().unwrap()[key] += bias;
    }

    fn add_linear_from(&mut self, other: &Self::LinearType) {
        for (u, bias) in other.iter() {
            self.add_linear(u.into(), *bias);
        }
    }
    fn add_quadratic_from(&mut self, other: &Self::QuadraticType) {
        for (u, v, bias) in other.iter_flat() {
            self.add_quadratic(u, v, bias);
        }
    }
    fn add_higher_order_from(&mut self, other: &Self::HigherOrderType) {
        for (key, bias) in other.iter() {
            self.add_higher_order_direct(key, *bias);
        }
    }

    fn add_quadratic_back(&mut self, u: Index, v: Index, bias: Bias) {
        let u_idx = self.add_variable(u);
        let v_idx = self.add_variable(u);
        self.enforce_quadratic();
        self.check_quadratic_dimensions(u_idx, v_idx);
        match (u_idx == v_idx, self.vartype(u)) {
            // -1*-1 == +1*+1 == 1 so this is a constant offset
            (true, Vtype::Spin) => self.offset += bias,
            // 1*1 == 1 and 0*0 == 0 so this is linear
            (true, Vtype::Binary) => self.linear[u_idx] += bias,
            (true, _) => self.quadratic.as_mut().unwrap()[u_idx].push(OneVarTerm::new(v, bias)),
            (false, _) => {
                let quadratic = self.quadratic.as_mut().unwrap();
                let mut insertion_idx = u_idx;
                if u_idx > v_idx {
                    insertion_idx = v_idx;
                }
                quadratic[insertion_idx].push(OneVarTerm::new(v, bias));
            }
        }
    }
    fn add_quadratic_from_dense(&mut self, dense: &[Bias], num_variables: Index) {
        self.add_active_variables(num_variables);
        self.enforce_quadratic();
        let f_add_quadratic = match self.is_linear() {
            true => Self::add_quadratic_back,
            false => Self::add_quadratic,
        };

        let nvars = num_variables.into();
        for u in 0..nvars {
            // diagonal
            f_add_quadratic(self, u.into(), u.into(), dense[u * (nvars + 1)]);
            // off-diagonal
            for v in (u + 1)..nvars {
                let qbias = dense[u * nvars + v] + dense[v * nvars + u];
                if qbias != Bias::default() {
                    f_add_quadratic(self, u.into(), v.into(), qbias);
                }
            }
        }
    }
}

impl<Index, Bias> ExpressionBaseSet<Index, Bias> for Expression<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    fn set_higher_order(&mut self, vars: &Vec<Index>, bias: Bias) {
        self.add_variables(vars);
        self.enforce_higher_order();
        let contributions = self.reduce_higher_order_vars(vars);
        match contributions.len() {
            0 => self.add_offset(bias),
            1 => self.add_linear(contributions[0], bias),
            2 => self.add_quadratic(contributions[0], contributions[1], bias),
            _ => self.higher_order.as_mut().unwrap()[&contributions] = bias,
        }
    }
}

impl<Index, Bias> ExpressionBaseMul<Index, Bias> for Expression<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    fn mul_offset(&mut self, lhs: Bias, rhs: Bias) {
        self.offset += lhs + rhs
    }

    fn mul_linear(&mut self, lhs: &Self::LinearType, rhs: &Self::LinearType) {
        for (u_idx, u_bias) in lhs.iter() {
            for (v_idx, v_bias) in rhs.iter() {
                self.add_quadratic(u_idx.into(), v_idx.into(), *u_bias * *v_bias);
            }
        }
    }

    fn mul_quadratic(&mut self, lhs: &Self::QuadraticType, rhs: &Self::QuadraticType) {
        for (lhs_u, lhs_v, lhs_bias) in lhs.iter_flat() {
            for (rhs_u, rhs_v, rhs_bias) in rhs.iter_flat() {
                self.add_higher_order(&vec![lhs_u, lhs_v, rhs_u, rhs_v], lhs_bias * rhs_bias);
            }
        }
    }

    fn mul_higher_order(&mut self, lhs: &Self::HigherOrderType, rhs: &Self::HigherOrderType) {
        for (lhs_ind, lhs_bias) in lhs.iter_contrib() {
            for (rhs_ind, rhs_bias) in rhs.iter_contrib() {
                let mut new_indices = lhs_ind.clone();
                new_indices.extend(rhs_ind);
                self.set_higher_order(&new_indices, *lhs_bias * *rhs_bias);
            }
        }
    }
}

impl<Index, Bias> ExpressionBaseMulDirect<Index, Bias> for Expression<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    fn mul_with_offset(&mut self, offset: Bias, v: Index, bias: Bias) {
        // Multiplying the offset with a variable creates a new linear term that is
        // added to the linear part of the expression. Thus we can reuse the add_linear
        // here.
        self.add_linear(v, bias * offset)
    }

    fn mul_with_linear(&mut self, linear: &Self::LinearType, v: Index, bias: Bias) {
        // Multiplying the linear part of the expression with a variable can produce
        // different results. For binary, this updates the linear term by multipliction
        // of the bias.
        // For spin, a new constant offset is created and the spin variable is removed
        // from the linear part.

        // Next, we itrate over all elements in the linear term and adjust the value
        // based on the logic.
        for (u_idx, u_bias) in linear.iter() {
            // We need to iterate over all elements, as we need to adjust the values,
            // in all cases.
            // However, we can make use of the logic already implemented in the
            // add_quadratic case. Which checks the logic based on the variable type.
            self.add_quadratic(u_idx.into(), v, *u_bias * bias)
        }
    }

    fn mul_with_quadratic(&mut self, quadratic: &Self::QuadraticType, v: Index, bias: Bias) {
        // Multiply the quadratic part with a variable.
        for (u, neighborhood) in quadratic.iter() {
            for term in neighborhood.iter() {
                self.add_higher_order(&vec![u.into(), term.index, v], term.bias * bias)
            }
        }
    }

    fn mul_with_higher_order(
        &mut self,
        higher_order: &Self::HigherOrderType,
        v: Index,
        bias: Bias,
    ) {
        for (indices, ho_bias) in higher_order.iter_contrib() {
            let mut new_indices = vec![v];
            new_indices.extend(indices);
            self.set_higher_order(&new_indices, *ho_bias * bias)
        }
    }
}

impl<Index, Bias> Expression<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    fn check_and_get(&self, v: Index) -> Result<usize, VariableOutOfRangeError> {
        match self.active[v.into()] {
            true => Ok(v.into()),
            false => Err(VariableOutOfRangeError(v.into())),
        }
    }

    fn check(&self, v: Index) -> Result<(), VariableOutOfRangeError> {
        match self.active[v.into()] {
            true => Ok(()),
            false => Err(VariableOutOfRangeError(v.into())),
        }
    }

    fn check_multi(&self, vars: &Vec<Index>) -> Result<(), VariableOutOfRangeError> {
        for v in vars {
            let v_idx: usize = (*v).into();
            if self.active[v_idx] {
                return Err(VariableOutOfRangeError(v_idx));
            }
        }
        Ok(())
    }

    pub fn enforce_quadratic(&mut self) {
        if !self.has_quadratic() {
            self.quadratic = Some(Quadratic::new(self.linear.len()))
        }
    }

    pub fn enforce_higher_order(&mut self) {
        if !self.has_higher_order() {
            self.higher_order = Some(HigherOrder::default())
        }
    }

    #[inline]
    pub fn has_quadratic(&self) -> bool {
        self.quadratic.is_some()
    }

    #[inline]
    pub fn has_higher_order(&self) -> bool {
        self.higher_order.is_some()
    }

    pub fn check_quadratic_dimensions(
        &self,
        u: usize,
        v: usize,
    ) -> Result<(), IndexOutOfOrderError> {
        let quadratic = self.quadratic.as_ref().unwrap();

        if quadratic[v].is_empty() || quadratic[v].last().unwrap().index <= u.into() {
            return Err(IndexOutOfOrderError(
                u,
                quadratic[v].last().unwrap().index.into(),
            ));
        }
        if quadratic[u].is_empty() || quadratic[u].last().unwrap().index <= v.into() {
            return Err(IndexOutOfOrderError(
                v,
                quadratic[u].last().unwrap().index.into(),
            ));
        }

        Ok(())
    }

    /// Assumes quadratic exists!
    /// Creates the bias if it doesn't already exist
    pub fn asymmetric_quadratic_ref(&mut self, u: Index, v: Index) -> &mut Bias {
        assert!(self.has_quadratic(), "quadratic is not initialized");
        let neighborhood: &mut Vec<OneVarTerm<Index, Bias>> = self
            .quadratic
            .as_mut()
            .and_then(|quadratic| quadratic.get_mut(u))
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

    fn reduce_higher_order_vars(&self, indices: &Vec<Index>) -> Vec<Index> {
        // We have a multiplicative interaction between multiple variables here,
        // similar to the case for quadratic. So we need to check the interactions
        // for each combination...
        // We need to efficiently check if indices exist more than once in the list
        // and if it is the case, we need to retrieve which indices are repeated, and
        // how often they are repeated.
        let mut ocurrences: HashMap<Index, usize> = HashMap::new();
        for index in indices.iter() {
            let value = ocurrences.get_mut(index);
            match value {
                Some(v) => *v += 1,
                None => {
                    let _ = ocurrences.insert(*index, 1);
                }
            }
        }
        let mut contribs: Vec<Index> = Vec::new();
        for (idx, count) in ocurrences.iter() {
            match (*count > 1, self.vartype(*idx)) {
                // Binary variables cancel out to a single binary variable.
                // Thus we can just add it once.
                (true, Vtype::Binary) => contribs.push(*idx),
                // Depending on the count, we have different behaviour.
                // Two spins will result in an offset.
                // So if we have exactly two spins, we just get the offset.
                // If we have three spins, it's offset + a single variable remaining
                // and thus contributing.
                // If we have four, it's offset once
                // s * s * s * s = (s * s) * (s * s) = 1 * 1 = 1
                // if we have five we have a single variable contributing
                //
                // thus in general, for an even number of contributions, we have
                // a single contrubution to the offset.
                // If we have an uneven number we have a offset contribution
                // plus a single variable contribution to the term.
                (true, Vtype::Spin) => {
                    if count % 2 == 1 {
                        contribs.push(*idx);
                    }
                }
                (true, _) => {
                    for _ in 0..*count {
                        contribs.push(*idx);
                    }
                }
                (false, _) => contribs.push(*idx),
            }
        }
        contribs
    }
}
