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

    /// Returns the linear terms of the expression.
    pub fn linear(&self) -> Result<Linear> {
        out!(expr_linear(@self))
    }

    /// Returns the quadratic terms of the expression.
    pub fn quadratic(&self) -> Result<Quadratic> {
        out!(expr_quadratic(@self))
    }

    /// Returns the higher-order terms of the expression.
    pub fn higher_order(&self) -> Result<HigherOrder> {
        out!(expr_higher_order(@self))
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
}
