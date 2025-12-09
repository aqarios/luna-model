use crate::traits::ContentEquality;

use super::Expression;

impl PartialEq for Expression {
    fn eq(&self, other: &Self) -> bool {
        if self.env.id() != other.env.id() {
            // Non-equal envs directly implicate non-equal expressions.
            return false;
        }
        if self.num_vars() != other.num_vars() {
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
        let eok = self.env.is_equal_contents(&other.env);
        let ook = self.offset == other.offset;
        let lok = self.linear == other.linear;
        let qok = self.quadratic == other.quadratic;
        let hok = self.higher_order == other.higher_order;
        eok && ook && lok && qok && hok
    }
}
