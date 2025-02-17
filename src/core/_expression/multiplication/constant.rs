use std::ops::AddAssign;

use crate::core::{
    higher_order_operations::TermC,
    operations::{Key, Term},
    term::{Constant, HigherOrder},
    Expression,
};

pub fn constant_times_expression(constant: &Constant, other: &Expression, result: &mut Expression) {
    constant_times_constant(constant, &other.constant, &mut result.constant);
    constant_times_term(constant, &other.linear, &mut result.linear);
    constant_times_term(constant, &other.quadratic, &mut result.quadratic);
    constant_times_higher_order(constant, &other.higher_order, &mut result.higher_order);
}

fn constant_times_constant(lhs: &Constant, rhs: &Constant, result: &mut Constant) {
    match (lhs.value, rhs.value) {
        // Both constants have a value, so we need to multiply and add to current constant
        (Some(l), Some(r)) => {
            result.add_assign(l * r);
        }
        (_, _) => (),
    }
}

pub fn constant_times_term<K: Key, T: Term<K>>(constant: &Constant, term: &T, result: &mut T) {
    if !constant.has_value() || !term.has_variables() {
        // Either the constant or the other term is 0.
        // Thus we do not need to perform any computations and can exit safely.
        return;
    }
    // Now we know that both constant and other have variables.
    let constval = constant.value.unwrap();
    if constval == 0.0 {
        // We also need to check if the constant value is 0. If the value is zero we can
        // also safely exit.
        return;
    }
    // Now we need to multiply each element of the term with the constant value and add it to the
    // result linear term.
    let mutresultvars = result.mutable_variables();
    for (key, val) in term.variables().iter() {
        let newval = val * constval;
        if newval == 0.0 {
            // The new value is 0 so the component in the term is zero.
            // We can safely skip this entry.
            continue;
        }
        // We need to insert the component to the variables of the result term.
        mutresultvars.insert(*key, newval);
    }
}

// The duplicate code is required as the `Term` trait cannot be implemented by HigherOrder
// as the HigherOrderKey does and cannot implement the Copy trait.
pub fn constant_times_higher_order(
    constant: &Constant,
    higher_order: &HigherOrder,
    result: &mut HigherOrder,
) {
    if !constant.has_value() || !higher_order.has_variables() {
        // Either the constant or the other term is 0.
        // Thus we do not need to perform any computations and can exit safely.
        return;
    }
    // Now we know that both constant and other have variables.
    let constval = constant.value.unwrap();
    if constval == 0.0 {
        // We also need to check if the constant value is 0. If the value is zero we can
        // also safely exit.
        return;
    }
    // Now we need to multiply each element of the term with the constant value and add it to the
    // result linear term.
    let mutresultvars = result.mutable_variables();
    for (key, val) in higher_order.variables().iter() {
        let newval = val * constval;
        if newval == 0.0 {
            // The new value is 0 so the component in the term is zero.
            // We can safely skip this entry.
            continue;
        }
        // We need to insert the component to the variables of the result term.
        mutresultvars.insert(key.clone(), newval);
    }
}
