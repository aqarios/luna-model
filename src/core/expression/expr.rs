use super::base::{
    ExpressionBase, ExpressionBaseAdd, ExpressionBaseAdjustment, ExpressionBaseCreation,
    ExpressionBaseMul, ExpressionBaseMulComponents, ExpressionBaseMulDirect, ExpressionBaseSet,
    ExpressionBaseTypes,
};
use super::VariableOutOfRangeErr;
use crate::core::environment::SharedEnvironment;
use crate::core::expression::errors::EnvMismatchError;
use crate::core::term::types::{OneVarTerm, OneVarTermConstruction, SizeType};
use crate::core::term::{HigherOrder, Linear, Quadratic};
use crate::core::traits::ContentEquality;
use crate::core::writer::ModelWriter;
use crate::core::{VarRef, Vtype};
use crate::types::{Bias, VarIndex};
use hashbrown::HashMap;
use itertools::Itertools;
use std::fmt::{Debug, Display, Formatter};

#[derive(Clone)]
pub struct Expression {
    pub env: SharedEnvironment,
    pub offset: Bias,
    pub linear: Linear,
    pub quadratic: Option<Quadratic>,
    pub higher_order: Option<HigherOrder>,
    /// Mirror of the linear array that tracks which variables are already
    /// contained in the expression. 1 indicates already added 0 indicating floating.
    pub active: Vec<bool>,
    pub num_variables: SizeType,
}

impl Expression {
    /// Deep clone the expression for a new environment.
    pub fn deep_clone(&self, env: SharedEnvironment) -> Self {
        Self {
            env,
            offset: self.offset,
            linear: self.linear.clone(),
            quadratic: self.quadratic.clone(),
            higher_order: self.higher_order.clone(),
            active: self.active.clone(),
            num_variables: self.num_variables.clone(),
        }
    }

    pub fn deep_clone_many(exprs: &[&Expression]) -> Result<Vec<Expression>, EnvMismatchError> {
        if exprs.len() == 0 {
            return Ok(Vec::new());
        }
        let new_env = {
            let old_env = &exprs[0].env;
            if !exprs.iter().all(|&e| e.env == *old_env) {
                Err(EnvMismatchError)
            } else {
                Ok(old_env.deep_clone())
            }
        }?;
        Ok(exprs
            .iter()
            .map(|e| e.deep_clone(new_env.clone()))
            .collect())
    }
}

impl Expression {
    pub fn items(&self) -> Vec<(Vec<VarIndex>, Bias)> {
        let mut constant = Vec::new();
        if self.offset != Bias::default() {
            constant.push((Vec::new(), self.offset));
        }
        let linear = self
            .linear
            .iter()
            .filter(|(_, &bias)| bias != Bias::default())
            .map(|(idx, &bias)| (vec![idx.into()], bias))
            .collect();
        let quadratic = match &self.quadratic {
            None => Vec::new(),
            Some(quad) => quad
                .iter_flat()
                .filter(|(_, _, bias)| bias != &Bias::default())
                .map(|(u_idx, v_idx, bias)| (vec![u_idx, v_idx], bias))
                .collect(),
        };
        let higher_order = match &self.higher_order {
            None => Vec::new(),
            Some(ho) => ho
                .iter_contrib()
                .filter(|(_, &bias)| bias != Bias::default())
                .map(|(contrib, &bias)| (contrib, bias))
                .collect(),
        };

        vec![constant, linear, quadratic, higher_order].concat()
    }

    pub fn linear_items(&self) -> Vec<(VarIndex, Bias)> {
        self.linear
            .iter()
            .filter(|(_, &bias)| bias != Bias::default())
            .map(|(idx, &bias)| (idx.into(), bias))
            .collect()
    }

    pub fn quadratic_items(&self) -> Vec<(VarIndex, VarIndex, Bias)> {
        match &self.quadratic {
            None => Vec::new(),
            Some(quad) => quad
                .iter_flat()
                .filter(|(_, _, bias)| bias != &Bias::default())
                .map(|(u_idx, v_idx, bias)| (u_idx, v_idx, bias))
                .collect(),
        }
    }

    pub fn higher_order_items(&self) -> Vec<(Vec<VarIndex>, Bias)> {
        match &self.higher_order {
            None => Vec::new(),
            Some(ho) => ho
                .iter_contrib()
                .filter(|(_, &bias)| bias != Bias::default())
                .map(|(contrib, &bias)| (contrib, bias))
                .collect(),
        }
    }

    pub fn contains(&self, needle: &VarRef) -> bool {
        for (indices, _) in self.items() {
            if indices.contains(&needle.id) {
                return true;
            }
        }
        false
    }

    pub fn is_constant(&self) -> bool {
        (self.linear.len() == 0) & !self.has_quadratic() & !self.has_higher_order()
    }
}

impl ExpressionBaseTypes for Expression {
    type LinearType = Linear;
    type QuadraticType = Quadratic;
    type HigherOrderType = HigherOrder;

    type HigherOrderKey = String;
}

impl ExpressionBaseCreation<VarIndex, Bias> for Expression {
    fn simple(env: SharedEnvironment, offset: Bias) -> Self {
        Self {
            env,
            offset,
            linear: Linear::default(),
            quadratic: None,
            higher_order: None,
            active: Vec::default(),
            num_variables: 0,
        }
    }
    fn empty(env: SharedEnvironment) -> Self {
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

    fn new(env: SharedEnvironment, active: Vec<bool>, num_variables: usize) -> Self {
        Self {
            env,
            offset: Bias::default(),
            linear: Linear::with_size(active.len()),
            quadratic: None,
            higher_order: None,
            active,
            num_variables,
        }
    }

    fn new_linear_single(env: SharedEnvironment, v: VarIndex, bias: Bias) -> Self {
        let linear = Linear::new_from_weighted_variable(v.into(), bias);
        // todo: make this it's own struct similar to linear.
        let mut active = Vec::new();
        active.resize(linear.len(), false);
        let v_idx: usize = v.into();
        active[v_idx] = true;

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

    fn new_linear_and_offset(
        env: SharedEnvironment,
        v: VarIndex,
        bias: Bias,
        offset: Bias,
    ) -> Self {
        let linear = Linear::new_from_weighted_variable(v.into(), bias);
        // todo: make this it's own struct similar to linear.
        let v_idx: usize = v.into();
        let mut active = Vec::new();
        active.resize(linear.len(), false);
        active[v_idx] = true;

        Self {
            env,
            offset,
            linear,
            quadratic: None,
            higher_order: None,
            active,
            num_variables: 1,
        }
    }

    fn new_linear(env: SharedEnvironment, u: (VarIndex, Bias), v: (VarIndex, Bias)) -> Self {
        let u0_idx: usize = u.0.into();
        let v0_idx: usize = v.0.into();
        let linear = Linear::new_from_variables((u0_idx, u.1), (v0_idx, v.1));
        // todo: make this it's own struct similar to linear.
        let mut active = Vec::new();
        active.resize(linear.len(), false);
        active[u0_idx] = true;
        active[v0_idx] = true;

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
    fn new_quadratic(env: SharedEnvironment, u: VarIndex, v: VarIndex, bias: Bias) -> Self {
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

impl ExpressionBaseAdjustment<VarIndex, Bias> for Expression {
    fn add_variable(&mut self, v: VarIndex) -> SizeType {
        // First, we need to check if the variable is larger or equal to the current
        // size of the linear terms.
        let size: usize = self.linear.len();
        let v_idx: usize = v.into();
        if v_idx >= size {
            // We can simply resize the linear term and the active tracker.
            self.linear.resize(v_idx + 1);
            self.active.resize(v_idx + 1, false);
            // We also need to resize the quadratic term if this expression has one.
            if self.has_quadratic() {
                self.quadratic.as_mut().unwrap().resize(v_idx + 1);
            }
            // We do not need to resize the higher order term as in this implementation
            // it grows dynamically.
            // ...
            // Now we set the currently added variable as active.
            self.active[v_idx] = true;
            // And incresae the variable counter by one.
            self.num_variables += 1;
        } else {
            // v < size
            // We do not need to do any resizing.
            // And can directly check if the variable is active.
            let active: bool = self.active[v_idx];
            // If the variable is already activated we need to do nothing.
            // This means the variable was already added at some point.
            // Otherwise, we need to activate it and increase the var counter by one.
            if !active {
                self.active[v_idx] = true;
                self.num_variables += 1;
            }
        }

        v_idx
    }

    fn remove_variable(&mut self, v: VarIndex) {
        let v_idx: usize = v.into();
        let size: usize = self.active.len();

        if v_idx == (size - 1) {
            self.active.resize(size - 1, false);
            self.linear.resize(size - 1);
            self.num_variables -= 1;
        } else if v_idx >= size {
            // self.active.resize(size, false);
            // self.linear.resize(size);
        } else {
            if self.active[v_idx] {
                // only reduce the variable count if the variable was active
                // before it's removal.
                self.num_variables -= 1;
            }
            self.active[v_idx] = false;
            self.linear[v_idx] = Bias::default();
        }
    }

    fn add_variables(&mut self, vars: &Vec<VarIndex>) {
        // We only need to call the add_variable for the largest index.
        // This will automatically allocate memory for all others.
        let max_index = vars.iter().max().unwrap();
        self.add_variable(*max_index);
        // Now we need to set each variable as active and increase the variable counter
        // if the variable has not been added before.
        for v in vars {
            let v_idx: usize = (*v).into();
            let active: bool = self.active[v_idx];
            if !active {
                self.active[v_idx] = true;
                self.num_variables += 1;
            }
        }
    }

    fn remove_variables(&mut self, vars: &Vec<VarIndex>) {
        for var in vars {
            self.remove_variable(*var);
        }
    }

    fn resize(&mut self, n: VarIndex) {
        if self.has_quadratic() {
            if <VarIndex as Into<usize>>::into(n) < self.linear.len() {
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
        self.active.resize(n.into(), false);

        // Again, higher order terms do not need to be resized, see `add_variables`

        assert!(
            !self.has_quadratic()
                || self.linear.len() == self.quadratic.as_ref().unwrap().len()
                || self.linear.len() == self.active.len()
        );
    }
}

impl ExpressionBase<VarIndex, Bias> for Expression {
    fn offset(&self) -> Bias {
        self.offset
    }

    fn linear(&self, v: VarIndex) -> Result<Bias, VariableOutOfRangeErr> {
        let v_idx = self.check_and_get(v)?;
        Ok(self.linear[v_idx])
    }

    fn quadratic(&self, u: VarIndex, v: VarIndex) -> Result<Bias, VariableOutOfRangeErr> {
        self.check_and_get(u)?;
        self.check_and_get(v)?;
        Ok(self
            .quadratic
            .as_ref()
            .map_or_else(Bias::default, |q| q[(u, v)]))
    }

    fn higher_order(&self, indices: &Vec<VarIndex>) -> Result<Bias, VariableOutOfRangeErr> {
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

    fn vtypes(&self) -> Vec<Vtype> {
        self.active
            .iter()
            .enumerate()
            .filter(|(_, &a)| a)
            .map(|(idx, _)| self.env.access().get_vtype(idx.into()))
            .unique()
            .collect_vec()
    }

    fn degree(&self) -> usize {
        let mut degree = 0;
        if !self.linear.is_zero() {
            // has a linear term -> at least degree 1.
            degree = 1;
        }
        if let Some(quad) = &self.quadratic {
            if quad.has_interaction() {
                // has a quadratic interaction -> at least deg 2
                degree = 2;
            }
        }
        if let Some(ho) = &self.higher_order {
            degree = ho.max_degree()
        }

        degree
    }

    fn variables(&self) -> Vec<VarIndex> {
        self.active
            .iter()
            .enumerate()
            .filter(|(_, &v)| v)
            .map(|(x, _)| x.into())
            .collect()
    }

    #[inline]
    fn vartype(&self, v: VarIndex) -> Vtype {
        self.env.access().get_vtype(v)
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

impl ExpressionBaseAdd<VarIndex, Bias> for Expression {
    fn add_offset(&mut self, bias: Bias) {
        self.offset += bias
    }

    fn add_linear(&mut self, v: VarIndex, bias: Bias) {
        self.add_variable(v);
        self.linear[v.into()] += bias;
    }

    fn add_quadratic(&mut self, u: VarIndex, v: VarIndex, bias: Bias) {
        self.add_variable(u);
        self.add_variable(v);
        self.enforce_quadratic();

        match (u == v, self.vartype(u)) {
            // -1*-1 == +1*+1 == 1 so this is constant offset
            (true, Vtype::Spin) => self.offset += bias,
            // 1*1 == 1 and 0*0 == 0 so this is linear
            (true, Vtype::Binary) => {
                self.linear[u.into()] += bias;
            }
            (_, _) => {
                if bias != Bias::default() {
                    *self.asymmetric_quadratic_ref(u, v) += bias;
                }
            }
        }
    }

    fn add_higher_order(&mut self, vars: &Vec<VarIndex>, bias: Bias) {
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

    fn add_linear_from(&mut self, other: &Self::LinearType, other_active: &Vec<bool>) {
        for (u, bias) in other.iter() {
            if self.active[u] || other_active[u] {
                self.add_linear(u.into(), *bias);
            }
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

    fn add_quadratic_back(&mut self, u: VarIndex, v: VarIndex, bias: Bias) {
        let u_idx = self.add_variable(u);
        let v_idx = self.add_variable(v);
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
    fn add_quadratic_from_dense(&mut self, dense: &[Bias], num_variables: VarIndex) {
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

impl ExpressionBaseSet<VarIndex, Bias> for Expression {
    fn set_higher_order(&mut self, vars: &Vec<VarIndex>, bias: Bias) {
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

impl ExpressionBaseMul<VarIndex, Bias> for Expression {
    fn multiply(lhs: &Self, rhs: &Self, result: &mut Self) {
        // First, we need to get the shared active variables for both
        // lhs and rhs.
        // This is required to determine which variables are actually to be
        // multiplied with eachother.
        // Let's say, we have the variables a, b, c, d, e, f
        // where a, b, f are used in expression 1.
        // and c, d, e are used in expression 2.
        //
        // Then, for expr1 the active variables vec is: [1, 1, 0, 0, 0, 1]
        // And, for expr2 the active variables vec is:  [0, 0, 1, 1, 1, 0]
        // Now, if we multiply the two linear terms. only the multiplications
        // where both are equal to one are used. So that we do not multiply a located
        // in expr1 with a located in expr2 which has 0 values as it is inactive.
        //
        // Also this is only important for linear. In the quadratic or higher order
        // should never be inactive variables.
        //
        let lhs_linear_actives: Vec<(VarIndex, Bias)> = lhs
            .linear
            .iter()
            .filter(|(idx, _)| lhs.active[*idx])
            .map(|(idx, bias)| (idx.into(), *bias))
            .collect();

        let rhs_linear_actives: Vec<(VarIndex, Bias)> = rhs
            .linear
            .iter()
            .filter(|(idx, _)| rhs.active[*idx])
            .map(|(idx, bias)| (idx.into(), *bias))
            .collect();

        // lhs      rhs
        // offset * offset              = rhs.offset * lhs.offset
        result.mul_offsets(&rhs.offset, &lhs.offset);
        // offset * linear              = rhs.linear * lhs.offset
        result.mul_linear_with_offset(&rhs_linear_actives, &lhs.offset);
        // linear * offset              = lhs.linear * rhs.offset
        result.mul_linear_with_offset(&lhs_linear_actives, &rhs.offset);
        // linear * linear              = lhs.linear * rhs.linear
        result.mul_linears(&lhs_linear_actives, &rhs_linear_actives);

        if lhs.has_quadratic() {
            let lhs_quad = lhs.quadratic.as_ref().unwrap();
            // quadratic * offset           = lhs.quadratic * rhs.offset
            result.mul_quadratic_with_offset(&lhs_quad, &rhs.offset);
            // quadratic * linear           = lhs.quadratic * rhs.linear
            result.mul_quadratic_with_linear(&lhs_quad, &rhs_linear_actives);

            if rhs.has_quadratic() {
                let rhs_quad = rhs.quadratic.as_ref().unwrap();
                // quadratic * quadratic        = lhs.quadratic * rhs.quadratic
                result.mul_quadratics(&lhs_quad, &rhs_quad);
            }

            if rhs.has_higher_order() {
                let rhs_ho = rhs.higher_order.as_ref().unwrap();
                // quadratic * higher_order     = rhs.higher_order * lhs.quadratic
                result.mul_higher_order_with_quadratic(&rhs_ho, &lhs_quad);
            }
        }

        if rhs.has_quadratic() {
            let rhs_quad = rhs.quadratic.as_ref().unwrap();
            // offset * quadratic           = rhs.quadratic * lhs.offset
            result.mul_quadratic_with_offset(&rhs_quad, &lhs.offset);
            // linear * quadratic           = rhs.quadratic * lhs.linear
            result.mul_quadratic_with_linear(&rhs_quad, &lhs_linear_actives);

            if lhs.has_higher_order() {
                let lhs_ho = lhs.higher_order.as_ref().unwrap();
                // higher_order * quadratic     = lhs.higher_order * rhs.quadratic
                result.mul_higher_order_with_quadratic(&lhs_ho, &rhs_quad);
            }
        }

        if lhs.has_higher_order() {
            let lhs_ho = lhs.higher_order.as_ref().unwrap();
            // higher_order * offset        = lhs.higher_order * rhs.offset
            result.mul_higher_order_with_offset(&lhs_ho, &rhs.offset);
            // higher_order * linear        = lhs.higher_order * rhs.linear
            result.mul_higher_order_with_linear(&lhs_ho, &rhs_linear_actives);

            if lhs.has_higher_order() && rhs.has_higher_order() {
                let rhs_ho = rhs.higher_order.as_ref().unwrap();
                // higher_order * higher_order  = lhs.higher_order * rhs.higher_order
                result.mul_higher_orders(&lhs_ho, &rhs_ho);
            }
        }

        if rhs.has_higher_order() {
            let rhs_ho = rhs.higher_order.as_ref().unwrap();
            // offset * higher_order        = rhs.higher_order * lhs.offset
            result.mul_higher_order_with_offset(&rhs_ho, &lhs.offset);
            // linear * higher_order        = rhs.higher_order * lhs.linear
            result.mul_higher_order_with_linear(&rhs_ho, &lhs_linear_actives);
        }
        result.cleanup()
    }
}

impl ExpressionBaseMulComponents<VarIndex, Bias> for Expression {
    fn mul_offsets(&mut self, lhs: &Bias, rhs: &Bias) {
        self.add_offset(*lhs * *rhs);
    }

    fn mul_linear_with_offset(&mut self, linear: &Vec<(VarIndex, Bias)>, offset: &Bias) {
        for (idx, bias) in linear.iter() {
            self.add_linear(*idx, *bias * *offset);
        }
    }

    fn mul_linears(&mut self, lhs: &Vec<(VarIndex, Bias)>, rhs: &Vec<(VarIndex, Bias)>) {
        for (u, u_bias) in lhs.iter() {
            for (v, v_bias) in rhs.iter() {
                self.add_quadratic(*u, *v, *u_bias * *v_bias);
            }
        }
    }

    fn mul_quadratic_with_offset(&mut self, lhs: &Self::QuadraticType, offset: &Bias) {
        for (u, v, bias) in lhs.iter_flat() {
            self.add_quadratic(u, v, *offset * bias);
        }
    }

    fn mul_quadratic_with_linear(
        &mut self,
        lhs: &Self::QuadraticType,
        rhs: &Vec<(VarIndex, Bias)>,
    ) {
        for (l, lbias) in rhs.iter() {
            for (u, v, qbias) in lhs.iter_flat() {
                self.add_higher_order(&vec![u, v, *l], *lbias * qbias);
            }
        }
    }

    fn mul_quadratics(&mut self, lhs: &Self::QuadraticType, rhs: &Self::QuadraticType) {
        for (lu, lv, lbias) in lhs.iter_flat() {
            for (ru, rv, rbias) in rhs.iter_flat() {
                self.add_higher_order(&vec![lu, lv, ru, rv], lbias * rbias);
            }
        }
    }

    fn mul_higher_order_with_offset(&mut self, lhs: &Self::HigherOrderType, offset: &Bias) {
        for (key, bias) in lhs.iter() {
            self.add_higher_order_direct(key, *bias * *offset);
        }
    }

    fn mul_higher_order_with_linear(
        &mut self,
        lhs: &Self::HigherOrderType,
        rhs: &Vec<(VarIndex, Bias)>,
    ) {
        for (contribs, hbias) in lhs.iter_contrib() {
            for (l, bias) in rhs.iter() {
                let mut loc_contribs = contribs.clone();
                loc_contribs.push(*l);
                self.add_higher_order(&loc_contribs, *bias * *hbias);
            }
        }
    }

    fn mul_higher_order_with_quadratic(
        &mut self,
        lhs: &Self::HigherOrderType,
        rhs: &Self::QuadraticType,
    ) {
        for (contribs, hbias) in lhs.iter_contrib() {
            for (u, v, qbias) in rhs.iter_flat() {
                let mut loc_contribs = contribs.clone();
                loc_contribs.push(u);
                loc_contribs.push(v);
                self.add_higher_order(&loc_contribs, qbias * *hbias);
            }
        }
    }

    fn mul_higher_orders(&mut self, lhs: &Self::HigherOrderType, rhs: &Self::HigherOrderType) {
        for (lhs_contribs, lhs_bias) in lhs.iter_contrib() {
            for (mut rhs_contribs, rhs_bias) in rhs.iter_contrib() {
                let mut loc_contribs = lhs_contribs.clone();
                loc_contribs.append(&mut rhs_contribs);
                self.add_higher_order(&loc_contribs, *lhs_bias * *rhs_bias);
            }
        }
    }
}

impl ExpressionBaseMulDirect<VarIndex, Bias> for Expression {
    fn mul_with_offset(&mut self, offset: Bias, v: VarIndex, bias: Bias) {
        // Multiplying the offset with a variable creates a new linear term that is
        // added to the linear part of the expression. Thus we can reuse the add_linear
        // here.
        self.add_linear(v, bias * offset)
    }

    fn mul_with_linear(&mut self, linear: &Self::LinearType, v: VarIndex, bias: Bias) {
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
            if self.active[u_idx] {
                self.add_quadratic(u_idx.into(), v, *u_bias * bias)
            }
        }
    }

    fn mul_with_quadratic(&mut self, quadratic: &Self::QuadraticType, v: VarIndex, bias: Bias) {
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
        v: VarIndex,
        bias: Bias,
    ) {
        for (indices, ho_bias) in higher_order.iter_contrib() {
            let mut new_indices = vec![v];
            new_indices.extend(indices);
            self.set_higher_order(&new_indices, *ho_bias * bias)
        }
    }
}

impl Expression {
    fn check_and_get(&self, v: VarIndex) -> Result<usize, VariableOutOfRangeErr> {
        let v_idx: usize = v.into();
        match v_idx <= self.active.len() {
            true => Ok(v_idx),
            false => Err(VariableOutOfRangeErr(v_idx)),
        }
    }

    fn check_multi(&self, vars: &Vec<VarIndex>) -> Result<(), VariableOutOfRangeErr> {
        for v in vars {
            let v_idx: usize = (*v).into();
            if !self.active[v_idx] {
                return Err(VariableOutOfRangeErr(v_idx));
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

    pub fn check_quadratic_dimensions(&self, u: usize, v: usize) {
        let quadratic = self.quadratic.as_ref().unwrap();
        assert!(
            quadratic[v].is_empty() || quadratic[v].last().unwrap().index <= u.into(),
            "VarIndex out of oder: last index <= {} (is {})",
            u,
            <VarIndex as Into<usize>>::into(quadratic[v].last().unwrap().index)
        );

        assert!(
            quadratic[u].is_empty() || quadratic[u].last().unwrap().index <= v.into(),
            "VarIndex out of oder: last index <= {} (is {})",
            v,
            <VarIndex as Into<usize>>::into(quadratic[u].last().unwrap().index)
        );
    }

    /// Assumes quadratic exists!
    /// Creates the bias if it doesn't already exist
    pub fn asymmetric_quadratic_ref(&mut self, u: VarIndex, v: VarIndex) -> &mut Bias {
        assert!(self.has_quadratic(), "quadratic is not initialized");
        &mut self.quadratic.as_mut().unwrap()[(u, v)]
    }

    fn reduce_higher_order_vars(&self, indices: &Vec<VarIndex>) -> Vec<VarIndex> {
        // We have a multiplicative interaction between multiple variables here,
        // similar to the case for quadratic. So we need to check the interactions
        // for each combination...
        // We need to efficiently check if indices exist more than once in the list
        // and if it is the case, we need to retrieve which indices are repeated, and
        // how often they are repeated.
        let mut ocurrences: HashMap<VarIndex, usize> = HashMap::new();
        for index in indices.iter() {
            let value = ocurrences.get_mut(index);
            match value {
                Some(v) => *v += 1,
                None => {
                    let _ = ocurrences.insert(*index, 1);
                }
            }
        }
        let mut contribs: Vec<VarIndex> = Vec::new();
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

    pub fn cleanup(&mut self) {
        if let Some(q) = &mut self.quadratic {
            q.cleanup();
            if q.is_empty() {
                self.quadratic = None
            }
        }
        if let Some(ho) = &mut self.higher_order {
            ho.cleanup();
            if ho.is_empty() {
                self.higher_order = None
            }
        }
    }
}

impl Debug for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let linear = ModelWriter::new()
            .write_linear(&self.env, &self.linear)
            .to_string();
        let quadratic = if let Some(q) = &self.quadratic {
            ModelWriter::new().write_quadratic(&self.env, q).to_string()
        } else {
            String::from("None")
        };
        let higher_order = if let Some(ho) = &self.higher_order {
            ModelWriter::new()
                .write_higher_order(&self.env, ho)
                .to_string()
        } else {
            String::from("None")
        };
        f.debug_struct("Expression")
            .field("environment_id", &self.env.id())
            .field("offset", &self.offset)
            .field("linear", &format_args!("{linear}"))
            .field("quadratic", &format_args!("{quadratic}"))
            .field("higher_order", &format_args!("{higher_order}"))
            .field("active", &self.active)
            .field("num_variables", &self.num_variables)
            .finish()
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = ModelWriter::new().write_expression(&self).to_string();
        f.write_str(&s)
    }
}

impl ContentEquality for Expression {
    fn is_equal_contents(&self, other: &Self) -> bool {
        self.env.is_equal_contents(&other.env)
            && self.offset == other.offset
            && self.linear == other.linear
            && self.quadratic == other.quadratic
            && self.higher_order == other.higher_order
            && self.active == other.active
            && self.num_variables == other.num_variables
    }
}
