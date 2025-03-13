use super::encodable::Encodable;
use super::versions::v0::SerConstraints as SerConstrV0;
use super::versions::v0::SerExpression as SerExprV0;
use crate::core::Constraints;
use crate::core::{Expression, VarId};

impl Encodable<SerExprV0> for Expression<VarId, f64> {}
impl Encodable<SerConstrV0> for Constraints<VarId, f64> {}
