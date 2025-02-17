use std::ops::AddAssign;

use crate::core::{
    higher_order_operations::TermC, term::HigherOrder, Environment, Expression, Vtype,
};

use super::{
    constant::constant_times_higher_order, linear::linear_times_higher_order,
    quadratic::quadratic_times_higher_order,
};

pub fn higher_order_times_expression(
    higher_order: &HigherOrder,
    other: &Expression,
    env: &Environment,
    result: &mut Expression,
) {
    // We can use the constant x higher_order to compute higher_order x constant.
    constant_times_higher_order(&other.constant, higher_order, &mut result.higher_order);
    // We can use linear x higher_order to compute higher_ordery x linear.
    linear_times_higher_order(&other.linear, higher_order, env, result);
    // We can use quadratic x higher_order to compute higher_ordery x quadratic.
    quadratic_times_higher_order(&other.quadratic, higher_order, env, result);
    higher_order_times_higher_order(higher_order, &other.higher_order, env, result);
}

fn higher_order_times_higher_order(
    lhs: &HigherOrder,
    rhs: &HigherOrder,
    env: &Environment,
    result: &mut Expression,
) {
    if !lhs.has_variables() || !rhs.has_variables() {
        // Any of the two elements of the product does not have variables and is thus empty and
        // treated as 0. Therefore, we have no result and do not need to edit the result
        // expression.
        return;
    }

    for (lhskey, lhsval) in lhs.variables().iter() {
        for (rhskey, rhsval) in rhs.variables().iter() {
            let newvalue = lhsval * rhsval;
            if newvalue == 0.0 {
                // We can skip this contribution safely.
                continue;
            }

            let lhsvars = HigherOrder::get_key_contributions(lhskey.clone());
            let rhsvars = HigherOrder::get_key_contributions(rhskey.clone());

            // All keys that are in both the lhs and the rhs contribution.
            // and the remaining variables.
            let mut shared_vars = Vec::new();
            let mut contributing_vars = Vec::new(); // the variables which are definitely in the
                                                    // new key.
            for var in lhsvars.iter() {
                if rhsvars.contains(var) {
                    shared_vars.push(*var);
                } else {
                    contributing_vars.push(*var);
                }
            }
            // If there are no shared keys, we create a new component containing both.
            if shared_vars.is_empty() {
                // shared vars does not contain anything, thus all variables already in the
                // contributing_vars variable.
                result.higher_order.add_multi(contributing_vars, newvalue);
                continue; // go to the next contribution.
            }
            // The shared vars contains variables that are in both components lhs and rhs.
            // Now, depending on the type, different things happen.
            //
            // If a binary variable is contained in the shared vars, this binary variable is in
            // both the lhs and rhs. However, we need it just once, as a binary variable
            // times itself is just itself, thus we can directly add it to the contributing_vars.
            //
            // If a real or int variable is contained in the shared vars, we need to add it twice,
            // once for the lhs and once for the rhs part. They do not cancel out etc. we need to
            // ensure the count is correctly reflected.
            //
            // If a spin variable is contained in the shared_vars, we can safely skip it as a
            // spin variable times itself results in +1, thus does not appear in the resulting
            // interaction and cancels out. We can safely ignore these.
            //
            // Let's do this and add the variables according to the above logic.
            for var in shared_vars {
                let vtype = env.get(&var).vtype;
                match vtype {
                    Vtype::Binary => {
                        // Add the variable once. var is binary: var * var = var.
                        contributing_vars.push(var)
                    }
                    Vtype::Spin => {
                        // Do nothing. var * var = +1.
                        continue;
                    }
                    _ => {
                        // var is either real or int. thus var * var = var * var.
                        // We need to add it twice.
                        contributing_vars.push(var);
                        contributing_vars.push(var);
                    }
                }
            }

            // Alright, all interacting variables are contained in the contributing_vars.
            // Let's create the contribution based on it's length. Can very well be anything from a
            // constant to a higher order component.
            match contributing_vars.len() {
                // cool!
                0 => result.constant.add_assign(newvalue),
                // yep, just a single variable left
                1 => result.linear.add_elem(contributing_vars[0], newvalue),
                // two variables left? That sounds quadratic!
                2 => {
                    result
                        .quadratic
                        .add_elem(contributing_vars[0], contributing_vars[1], newvalue)
                }
                // well, back to the higher order stuff :(
                _ => result.higher_order.add_multi(contributing_vars, newvalue),
            }
        }
    }
}
