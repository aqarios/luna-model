use std::ops::AddAssign;

use crate::core::{
    higher_order_operations::TermC,
    operations::Term,
    term::{HigherOrder, Linear, Quadratic, QuadraticKeyContains},
    Environment, Expression, Vtype,
};

use super::constant::constant_times_term;

pub fn linear_times_expression(
    linear: &Linear,
    other: &Expression,
    env: &Environment,
    result: &mut Expression,
) {
    // We can use the constant x linear to compute linear x constant.
    constant_times_term(&other.constant, linear, &mut result.linear);
    linear_times_linear(linear, &other.linear, env, result);
    linear_times_quadratic(linear, &other.quadratic, env, result);
    linear_times_higher_order(linear, &other.higher_order, env, result);
}

fn linear_times_linear(lhs: &Linear, rhs: &Linear, env: &Environment, result: &mut Expression) {
    if !lhs.has_variables() || !rhs.has_variables() {
        // Any of the two elements of the product does not have variables and is thus empty and
        // treated as 0. Therefore, the result is a new empty Linear term and no Quadratic
        // term is produced and we do not need to alter the result expression.
        return;
    }

    for (lhskey, lhsval) in lhs.variables().iter() {
        for (rhskey, rhsval) in rhs.variables().iter() {
            let newvalue = lhsval * rhsval;
            if newvalue == 0.0 {
                // If the newvalue is 0 we can skip the combination.
                continue;
            }
            let vtype = env.get(lhskey).vtype;
            let is_keys_equal = lhskey == rhskey;
            match (is_keys_equal, vtype) {
                (true, Vtype::Binary) => {
                    // Remains a linear term as x^2 = x for x = 1 and x = 0.
                    // We can just append the variable to the result's linear term.
                    result.linear.add_elem(*lhskey, newvalue);
                }
                (true, Vtype::Spin) => {
                    // Results in a constant offset of value which is added to the constant offset
                    // of the result expression.
                    // (-1)^2 = 1 and (+1)^2 = 1. So regardless of the result, we know it's +1 and
                    // thus we can treat it as a constant offset.
                    result.constant.add_assign(newvalue);
                }
                (_, _) => {
                    // A new quadratic component is generated and needs to be added to the
                    // quadratic term of the result expression.
                    result.quadratic.add_elem(*lhskey, *rhskey, newvalue);
                }
            }
        }
    }
}

pub fn linear_times_quadratic(
    lin: &Linear,
    quad: &Quadratic,
    env: &Environment,
    result: &mut Expression,
) {
    if !lin.has_variables() || !quad.has_variables() {
        // Any of the two elements of the product does not have variables and is thus empty and
        // treated as 0. Therefore, we have no result and do not need to edit the result
        // expression.
        return;
    }

    for (linkey, linval) in lin.variables().iter() {
        for (quadkey, quadval) in quad.variables().iter() {
            let newvalue = linval * quadval;
            if newvalue == 0.0 {
                // If the newvalue is 0 we can skip the combination.
                continue;
            }
            let vtype = env.get(linkey).vtype;
            let is_contained = quadkey.contained(*linkey);
            match (is_contained, vtype) {
                (Some(_), Vtype::Binary) => {
                    // linear key is equal to one of the quadratic keys and the quadratic component
                    // remains quadratic. We can thus just update the result expression with the
                    // current quadratic component.
                    result.quadratic.add_kv(*quadkey, newvalue);
                }
                (Some(c), Vtype::Spin) => {
                    // We have a spin. Thus we have something like: l * a * b where l is either
                    // equal to a or b.
                    // We have some other information regarding quadratic keys with spin variables.
                    // We know for sure that the two variables can not be equal in this case, i.e.
                    // a != b. As if a and b (the two variables in the quadratic component) would
                    // be equal it would result in a constant term. This is caught earlier. E.g. a
                    // quadratic term is produced by multiplying two variables or two linear
                    // terms. In both cases, if two equal spin variables are multiplied a constant
                    // is produced. Thus this can and should never happen. Just to make sure let's
                    // add a panic. As if this would happen we have a major error in the code.
                    if c.not_contained.is_none() {
                        panic!("Quadratic entry for the same spin variable. This should be a constant offset.")
                    }
                    // Alright, now we can work with the components. We have one variable in the
                    // quadratic component that is spin and one that is something else and we have
                    // the linear component which is also a linear variable.
                    // The linear variable times the contained quadratic spin variable results in a
                    // in +1 in all cases. Thus, we remain with a linear term for the other
                    // variable, i.e. the `not_contained` variable.
                    result.linear.add_elem(c.not_contained.unwrap(), newvalue);
                }
                (_, _) => {
                    // In all other paths a higher order component is generated.
                    let (a, b) = Quadratic::get_key_contributions(quadkey);
                    result.higher_order.add_elem(*linkey, a, b, newvalue);
                }
            }
        }
    }
}

pub fn linear_times_higher_order(
    lin: &Linear,
    ho: &HigherOrder,
    env: &Environment,
    result: &mut Expression,
) {
    if !lin.has_variables() || !ho.has_variables() {
        // Any of the two elements of the product does not have variables and is thus empty and
        // treated as 0. Therefore, we have no result and do not need to edit the result
        // expression.
        return;
    }

    for (linkey, linval) in lin.variables().iter() {
        for (hokey, hoval) in ho.variables().iter() {
            let newvalue = linval * hoval;
            if newvalue == 0.0 {
                // If the newvalue is 0 we can skip the combination.
                continue;
            }
            let vtype = env.get(linkey).vtype;
            let hokeyelems = HigherOrder::get_key_contributions(hokey.clone());
            let is_lin_contained = hokeyelems.contains(linkey);
            match (is_lin_contained, vtype) {
                (true, Vtype::Binary) => {
                    // If the linear key is contained in the higher order component,
                    // we can directly add the current higher order component with the new value
                    // As two equal binary variables result in itself.
                    result.higher_order.add_kv(hokey.clone(), newvalue);
                }
                (true, Vtype::Spin) => {
                    // As it is a spin variable it might result in a quadratic component depending
                    // on the number of variables in the current higher order term.
                    // If we have just three variables of which one is a spin. We know that the
                    // multiplication will result in the two spin variables to result in +1.
                    // Thus only two variables are left and we have a quadratic component with the
                    // updated value.
                    // If we have more than three keys (four and more) it will remain a higher
                    // order term.
                    let subset = hokeyelems
                        .iter()
                        .filter(|e| *e != linkey)
                        .map(|&e| e)
                        .collect::<Vec<_>>();
                    match subset.len() {
                        // not possible but who knows...
                        0 => result.constant.add_assign(newvalue),
                        // yep, just a single variable left
                        1 => result.linear.add_elem(subset[0], newvalue),
                        // two variables left? That sounds quadratic!
                        2 => result.quadratic.add_elem(subset[0], subset[1], newvalue),
                        // well, back to the higher order stuff :(
                        _ => result.higher_order.add_multi(subset, newvalue),
                    }
                }
                (_, _) => {
                    // New higher order component with the current higher order keys and the
                    // key from the linear component.
                    let newkey = HigherOrder::update_key(hokey.clone(), *linkey);
                    result.higher_order.add_kv(newkey, newvalue);
                }
            }
        }
    }
}
