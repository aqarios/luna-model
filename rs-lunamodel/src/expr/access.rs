use lunamodel_capi::{LmHigherOrder, LmQuadratic};
use lunamodel_types::Vtype;

use crate::{Env, HigherOrder, Linear, Quadratic, VarRef, error::Result};

use super::Expression;

impl Expression {
    /// Returns a handle to the shared environment.
    pub fn env(&self) -> Result<Env> {
        out!(expr_env(@self))
    }

    /// Returns the constant offset.
    pub fn offset(&self) -> Result<f64> {
        out!(expr_offset(@self))
    }

    /// Replaces the constant offset while keeping all variable terms unchanged.
    pub fn set_offset(&mut self, value: f64) -> Result<()> {
        check!(expr_set_offset(@mut self, value))
    }

    /// Returns the number of distinct variables.
    pub fn num_vars(&self) -> Result<usize> {
        out!(expr_num_vars(@self))
    }

    /// Returns the degree of the expression.
    pub fn degree(&self) -> Result<usize> {
        out!(expr_degree(@self))
    }

    /// Returns whether the expression is a plain constant.
    pub fn is_constant(&self) -> Result<bool> {
        out!(expr_is_constant(@self))
    }

    /// Returns whether any quadratic term is present.
    pub fn has_quadratic(&self) -> Result<bool> {
        out!(expr_has_quadratic(@self))
    }

    /// Returns whether any higher-order term is present.
    pub fn has_higher_order(&self) -> Result<bool> {
        out!(expr_has_higher_order(@self))
    }

    /// Returns the distinct variable types used by the expression.
    pub fn vtypes(&self) -> Result<Vec<Vtype>> {
        out!(expr_vtypes(@self))
    }

    /// Returns a copy of the expression's complete linear term storage.
    pub fn linear_term(&self) -> Result<Linear> {
        out!(expr_linear(@self))
    }

    /// Returns the quadratic terms of the expression.
    pub fn quadratic_term(&self) -> Result<Option<Quadratic>> {
        out!(expr_quadratic(@self), |handle: *mut LmQuadratic| {
            if handle.is_null() {
                Ok(None)
            } else {
                Ok(Some(Quadratic::from_raw(handle)))
            }
        })
    }

    /// Returns the higher-order terms of the expression.
    pub fn higher_order_term(&self) -> Result<Option<HigherOrder>> {
        out!(expr_higher_order(@self), |handle: *mut LmHigherOrder| {
            if handle.is_null() {
                Ok(None)
            } else {
                Ok(Some(HigherOrder::from_raw(handle)))
            }
        })
    }

    /// Returns the distinct variables referenced by the expression, excluding
    /// inverted binary helper variables.
    pub fn vars(&self) -> Result<Vec<VarRef>> {
        out!(expr_vars(@self))
    }

    /// Returns every variable referenced by the expression, including the
    /// inverted binary helper variables that [`vars`](Self::vars) filters out.
    pub fn complete_vars(&self) -> Result<Vec<VarRef>> {
        out!(expr_complete_vars(@self))
    }

    pub fn raw_linear_items(&self) -> Result<Vec<(u32, f64)>> {
        self.linear_term()?.items()
    }

    pub fn raw_quadratic_items(&self) -> Result<Vec<(u32, u32, f64)>> {
        match self.quadratic_term()? {
            Some(terms) => terms.items(),
            None => Ok(Vec::new()),
        }
    }

    pub fn raw_higher_order_items(&self) -> Result<Vec<(Vec<u32>, f64)>> {
        match self.higher_order_term()? {
            Some(terms) => terms.items(),
            None => Ok(Vec::new()),
        }
    }

    pub fn linear_items(&self) -> Result<Vec<(VarRef, f64)>> {
        let env = self.env()?;
        self.raw_linear_items()?
            .into_iter()
            .map(|(idx, bias)| Ok((VarRef::new(idx, env.try_clone()?)?, bias)))
            .collect()
    }

    pub fn quadratic_items(&self) -> Result<Vec<(VarRef, VarRef, f64)>> {
        let env = self.env()?;
        self.raw_quadratic_items()?
            .into_iter()
            .map(|(u, v, bias)| {
                Ok((
                    VarRef::new(u, env.try_clone()?)?,
                    VarRef::new(v, env.try_clone()?)?,
                    bias,
                ))
            })
            .collect()
    }

    pub fn higher_order_items(&self) -> Result<Vec<(Vec<VarRef>, f64)>> {
        let env = self.env()?;
        self.raw_higher_order_items()?
            .into_iter()
            .map(|(indices, bias)| {
                let vars = indices
                    .into_iter()
                    .map(|idx| VarRef::new(idx, env.try_clone()?))
                    .collect::<Result<_>>()?;
                Ok((vars, bias))
            })
            .collect()
    }

    pub fn items(&self) -> Result<Vec<(Vec<VarRef>, f64)>> {
        let mut items = self
            .linear_items()?
            .into_iter()
            .map(|(var, bias)| (vec![var], bias))
            .collect::<Vec<_>>();
        items.extend(
            self.quadratic_items()?
                .into_iter()
                .map(|(u, v, bias)| (vec![u, v], bias)),
        );
        items.extend(self.higher_order_items()?);
        let offset = self.offset()?;
        if offset != 0.0 {
            items.push((Vec::new(), offset));
        }
        Ok(items)
    }

    pub fn linear(&self, idx: u32) -> Result<f64> {
        self.linear_term()?.index(idx)
    }

    pub fn quadratic(&self, u: u32, v: u32) -> Result<f64> {
        match self.quadratic_term()? {
            Some(terms) => terms.index(u, v),
            None => Ok(0.0),
        }
    }

    pub fn higher_order(&self, vars: &[u32]) -> Result<f64> {
        match self.higher_order_term()? {
            Some(terms) => terms.index(vars),
            None => Ok(0.0),
        }
    }

    pub fn contains(&self, var: &VarRef) -> Result<bool> {
        out!(expr_contains(@self, @var))
    }
}
