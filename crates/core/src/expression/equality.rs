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
    fn equal_contents(&self, other: &Self) -> bool {
        // if !self.env.equal_contents(&other.env) {
        //     return false;
        // }
        for (vars, bias) in self.items() {
            match &vars[..] {
                [] => {
                    if bias != other.offset {
                        return false;
                    }
                }
                [u] => {
                    let othervar = other.env.lookup(&u.name().unwrap()).ok();
                    match othervar {
                        Some(o) => {
                            if bias != other.linear(o.id) {
                                return false;
                            }
                        }
                        None => return false,
                    }
                }
                [u, v] => {
                    let othervar_u = other.env.lookup(&u.name().unwrap()).ok();
                    let othervar_v = other.env.lookup(&v.name().unwrap()).ok();
                    match (othervar_u, othervar_v) {
                        (Some(ou), Some(ov)) => {
                            if bias != other.quadratic(ou.id, ov.id) {
                                return false;
                            }
                        }
                        _ => return false,
                    }
                }
                vars => {
                    let mut others = Vec::new();
                    for v in vars.iter() {
                        let ov = other.env.lookup(&v.name().unwrap()).ok();
                        match ov {
                            Some(o) => others.push(o.id),
                            None => return false,
                        }
                    }
                    if bias != other.higher_order(&others) {
                        return false;
                    }
                }
            }
        }

        true
    }
}
