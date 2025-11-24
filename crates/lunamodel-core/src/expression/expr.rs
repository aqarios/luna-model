use std::fmt::Debug;

use bitvec::vec::BitVec;
use lunamodel_types::Bias;

use crate::ArcEnv;

use super::term::{HigherOrder, Linear, Quadratic};

/// A mathematical Expression of arbitrary degree.
#[derive(Debug, Clone)]
pub struct Expression {
    /// The [Environment] as an [Arc<RwLock<_>>].
    env: ArcEnv,
    /// The constant offset ([Bias]).
    offset: Bias,
    /// The [Linear] terms of this [Expression].
    linear: Linear,
    /// The [Quadratic] terms of this [Expression].
    quadratic: Option<Quadratic>,
    /// The [HigherOrder] terms of this [Expression].
    higher_order: Option<HigherOrder>,
    /// The number of variables in this [Expression].
    num_vars: usize,

    /// Utility lookup [BitVec] to indicate which variables are used within
    /// the [Expression].
    active: BitVec,
}

impl Expression {}

// Custom [Debug] implementation, todo.
// impl<'e> Debug for Expression<'e> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//
//     }
// }

impl PartialEq for Expression {
    fn eq(&self, other: &Self) -> bool {
        if self.env.id() != other.env.id() {
            // Non-equal envs directly implicate non-equal expressions.
            return false;
        }
        if self.num_vars != other.num_vars {
            // Non-equal number of active variables, cannot be identical.
            return false;
        }
        if self.linear != other.linear {
            return false;
        }
        let quads_eq = match (self.quadratic.is_some(), other.quadratic.is_some()) {
            (true, true) => self.quadratic.as_ref().unwrap() == other.quadratic.as_ref().unwrap(),
            (false, false) => true,
            _ => false,
        };
        if !quads_eq {
            return false;
        }
        let ho_eq = match (self.higher_order.is_some(), other.higher_order.is_some()) {
            (true, true) => {
                self.higher_order.as_ref().unwrap() == other.higher_order.as_ref().unwrap()
            }
            (false, false) => true,
            _ => false,
        };
        if !ho_eq {
            return false;
        }
        // Everything has to be eq at this point. Otherwise, we would have returned
        // earlier
        true
    }
}
