use crate::traits::ContentEquality;

use super::Expression;

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

impl ContentEquality for Expression {
    fn is_equal_contents(&self, other: &Self) -> bool {
        _ = other;
        unimplemented!()
    }
}
