use crate::core::{Constraint, Expression, VarId};

pub type Expr = Expression<VarId, f64>;
pub type Constr = Constraint<VarId, f64>;
