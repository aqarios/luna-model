use super::{Expression, ExpressionBase};

impl PartialEq for Expression {
    fn eq(&self, other: &Self) -> bool {
        if self.env.id() != other.env.id() {
            // Non-equal envs directly implicate non-equal expressions.
            return false;
        }
        if self.num_variables() != other.num_variables() {
            // Non-equal number of active variables, cannot be identical.
            return false;
        }
        if self.linear != other.linear {
            return false;
        }
        let quads_eq = match (self.has_quadratic(), other.has_quadratic()) {
            (true, true) => self.quadratic.as_ref().unwrap() == other.quadratic.as_ref().unwrap(),
            (false, false) => true,
            _ => false,
        };
        if !quads_eq {
            return false;
        }
        let ho_eq = match (self.has_higher_order(), other.has_higher_order()) {
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
