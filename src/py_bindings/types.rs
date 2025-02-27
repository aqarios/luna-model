use crate::core::{constraints::Constraints, Constraint, Expression, VarId};

pub type Expr = Expression<VarId, f64>;
pub type Constr = Constraint<VarId, f64>;
pub type Constrs = Constraints<VarId, f64>;
