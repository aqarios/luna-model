use crate::core::term::higher_order::HigherOrderKeyContains;
use crate::core::{
    higher_order_operations::TermC,
    operations::{Key, Term},
    term::{Constant, HigherOrder, Linear, Quadratic},
    variable::VarId,
    Environment, Vtype,
};
// Constant
pub fn constant_times_constant(lhs: &Constant, rhs: &Constant) -> Constant {
    match (lhs.value, rhs.value) {
        (Some(l), Some(r)) => Constant::new(l * r),
        (_, _) => Constant::empty(),
    }
}

pub fn constant_times_term<V: Key, T: Term<V>>(lhs: &Constant, rhs: &T) -> T {
    if lhs.value.is_none() {
        // The value of the constant is None which is equivalent to being 0.
        // Thus we generate an empty term.
        return T::empty(rhs.env_id());
    }
    let value = lhs.value.unwrap();
    // The value of the constant might still be 0. So we need to check.
    if value == 0.0 {
        // Same case as above.
        return T::empty(rhs.env_id());
    }
    // If the value of the constant is 1 we can return a clone/copy of the term.
    if value == 1.0 {
        return T::new_from_other(&rhs);
    }
    // If we reach here we need to do actual work, i.e., multiply each element of the term
    // with the constant value.

    let mut out = T::new_from_other(&rhs);
    let outvars = out.mutable_variables();

    // We iterate over the rhs variables and mutate the outvars variables.
    for (linkey, linval) in rhs.variables().iter() {
        // We know for sure that for every linkey there exists an entry in the outvars.
        // As both contain the exact same variables, they are carbon copies of eachother.
        // Now we check if the new value is zero. Which can happen in multiplications.
        // To enhance possible further processing we remove the entry from the term in
        // case it is now zero.
        let newvalue = linval * value;
        if newvalue == 0.0 {
            // Remove now 0 variable from the term.
            outvars.remove(linkey);
        } else {
            // Update the value of the variable in the term.
            let mutoutvar = outvars.get_mut(linkey).unwrap();
            *mutoutvar = newvalue;
        }
    }

    out
}
// pub fn constant_times_quadratic(lhs: &Constant, rhs: &Quadratic) -> Linear {
//     unimplemented!()
// }
// The duplicate code is required as the `Term` trait cannot be implemented by HigherOrder
// as the HigherOrderKey does and cannot implement the Copy trait.
pub fn constant_times_higher_order(lhs: &Constant, rhs: &HigherOrder) -> HigherOrder {
    if lhs.value.is_none() {
        // The value of the constant is None which is equivalent to being 0.
        // Thus we generate an empty term.
        return HigherOrder::empty(rhs.env_id);
    }
    let value = lhs.value.unwrap();
    // The value of the constant might still be 0. So we need to check.
    if value == 0.0 {
        // Same case as above.
        return HigherOrder::empty(rhs.env_id);
    }
    // If the value of the constant is 1 we can return a clone/copy of the term.
    if value == 1.0 {
        return HigherOrder::new_from_other(&rhs);
    }
    // If we reach here we need to do actual work, i.e., multiply each element of the term
    // with the constant value.

    let mut out = HigherOrder::new_from_other(&rhs);
    let outvars = out.mutable_variables();

    // We iterate over the rhs variables and mutate the outvars variables.
    for (linkey, linval) in rhs.variables().iter() {
        // We know for sure that for every linkey there exists an entry in the outvars.
        // As both contain the exact same variables, they are carbon copies of eachother.
        // Now we check if the new value is zero. Which can happen in multiplications.
        // To enhance possible further processing we remove the entry from the term in
        // case it is now zero.
        let newvalue = linval * value;
        if newvalue == 0.0 {
            // Remove now 0 variable from the term.
            outvars.remove(linkey);
        } else {
            // Update the value of the variable in the term.
            let mutoutvar = outvars.get_mut(linkey).unwrap();
            *mutoutvar = newvalue;
        }
    }

    out
}

// Linear
// pub fn linear_times_constant(lhs: &Linear, rhs: &Constant) -> Linear {
//     unimplemented!()
// }
pub fn linear_times_linear(
    lhs: &Linear,
    rhs: &Linear,
    env: &Environment,
) -> (Linear, Option<Quadratic>) {
    // Computes lhs * rhs.
    // Thus, if lhs is empty, the result is empty.
    // If rhs is empty the result is also empty. As empty is equal to 0.
    if !lhs.has_variables() || !rhs.has_variables() {
        // Any of the two elements of the product does not have variables and is thus empty and
        // treated as 0. Therefore, the result is a new empty Linear term and no Quadratic
        // term is produced.
        return (Linear::empty(lhs.env_id), None);
    }

    // From here on, both terms are not 0. Thus we can do the computations.
    // The multiplication of two linear terms can still be a linear term in case the variables
    // are equal and binary or spin. If they are different variables a quadratic term is generated.
    let mut linout = Linear::empty(lhs.env_id);
    let mut quadout: Option<Quadratic> = None;

    for (lhskey, lhsvalue) in lhs.variables().iter() {
        for (rhskey, rhsvalue) in rhs.variables().iter() {
            let newvalue = lhsvalue * rhsvalue;
            // If the newvalue is 0. Nothing happens regardless of the following logic.
            if newvalue == 0.0 {
                continue;
            }
            // The newvalue is not 0. So we need to do actual work.
            let vtype = env.get(lhskey).vtype;
            if lhskey == rhskey && (vtype == Vtype::Binary || vtype == Vtype::Spin) {
                // If the two keys are equal and have type Binary or Spin, it results in a linear entry
                // We can choose any of the two keys as the new key.
                linout.add_elem(*lhskey, newvalue);
            } else {
                // Otherwise, a quadratic entry is produced.
                if quadout.is_none() {
                    // No quadratic term has been generated yet. Thus, we need to genereate a new
                    quadout = Some(Quadratic::new_from_keys_with_value(
                        lhs.env_id, lhskey, rhskey, newvalue,
                    ));
                } else {
                    // We need to add the new entry to the quadratic term
                    quadout.as_mut().unwrap().add_elem(lhskey, rhskey, newvalue);
                }
            }
        }
    }

    (linout, quadout)
}
pub fn linear_times_quadratic(
    lin: &Linear,
    quad: &Quadratic,
    env: &Environment,
) -> (Quadratic, Option<HigherOrder>) {
    // Computes lhs * rhs.
    // Thus, if lhs is empty, the result is empty.
    // If rhs is empty the result is also empty. As empty is equal to 0.
    if !lin.has_variables() || !quad.has_variables() {
        // Any of the two elements of the product does not have variables and is thus empty and
        // treated as 0. Therefore, the result is a new empty quadratic term and no higher order
        // term is produced.
        return (Quadratic::empty(lin.env_id), None);
    }

    let mut quadout = Quadratic::empty(lin.env_id);
    let mut hoout: Option<HigherOrder> = None;

    for (linkey, linval) in lin.variables().iter() {
        for (quadkey, quadval) in quad.variables().iter() {
            let newvalue = linval * quadval;
            // If the newvalue is 0. Nothing happens regardless of the following logic.
            if newvalue == 0.0 {
                continue;
            }
            // The newvalue is not 0. So we need to do actual work.
            // If the linkey is contained in the quadkey and the variable type is Binary or Spin
            // a quadratic entry is generated.
            let vtype = env.get(linkey).vtype;
            let (quadcontrib_a, quadcontrib_b) = Quadratic::get_key_contributions(quadkey);
            if (quadcontrib_a == *linkey || quadcontrib_b == *linkey)
                && (vtype == Vtype::Binary || vtype == Vtype::Spin)
            {
                // The linkey is contained in the quadkey and we have binary or spin type.
                // Thus we generate a quadratic entry.
                quadout.add_elem(&quadcontrib_a, &quadcontrib_b, newvalue);
            } else {
                // linkey is not contained or not of type Binary or Spin.
                // Thus we generate a new higher order entry.
                if hoout.is_none() {
                    // Generate a new HigherOrder term.
                    hoout = Some(HigherOrder::new_from_keys_with_value(
                        lin.env_id,
                        *linkey,
                        quadcontrib_a,
                        quadcontrib_b,
                        newvalue,
                    ))
                } else {
                    // We need to update the current higher order term with the new entry.
                    hoout.as_mut().unwrap().add_elem(
                        *linkey,
                        quadcontrib_a,
                        quadcontrib_b,
                        newvalue,
                    );
                }
            }
        }
    }

    (quadout, hoout)
}
pub fn linear_times_higher_order(lin: &Linear, ho: &HigherOrder, env: &Environment) -> HigherOrder {
    // Computes lhs * rhs.
    // Thus, if lhs is empty, the result is empty.
    // If rhs is empty the result is also empty. As empty is equal to 0.
    if !lin.has_variables() || !ho.has_variables() {
        // Any of the two elements of the product does not have variables and is thus empty and
        // treated as 0. Therefore, the result is a new empty quadratic term and no higher order
        // term is produced.
        return HigherOrder::empty(lin.env_id);
    }

    // Multiplication with a higher order term is easier in the sense that HigherOrder is the
    // highest order representation of a term in aq-models. Thus we just need to update the
    // entries in what was given to us.
    let mut hoout = HigherOrder::empty(lin.env_id);

    for (linkey, linval) in lin.variables().iter() {
        for (hokey, hoval) in ho.variables().iter() {
            let newvalue = linval * hoval;
            // If the newvalue is 0. Nothing happens regardless of the following logic.
            // I.e., we do not generate an entry. In case we would start from a carbon
            // copy of the ho state. We would have to remove this value from the hoout.
            if newvalue == 0.0 {
                continue;
            }
            // The newvalue is not 0. So we need to do actual work.
            // If the linkey is contained in the hokey and the variable type is Binary or Spin
            // the current higher order entry is updated.
            let hocontribs = HigherOrder::get_key_contributions(hokey.to_string());
            let vtype = env.get(linkey).vtype;
            if HigherOrder::key_contains_other(hocontribs, *linkey)
                && (vtype == Vtype::Binary || vtype == Vtype::Spin)
            {
                // We can safely update/insert with the newvalue for the current ho key.
                hoout.add_kv(hokey.to_string(), newvalue);
            } else {
                // A new higher order entry is generated.
                let newkey = HigherOrder::update_key(hokey.to_string(), *linkey);
                hoout.add_kv(newkey, newvalue);
            }
        }
    }

    hoout
}

// Quadratic
// pub fn quadratic_times_constant(lhs: &Quadratic, rhs: &Constant) -> Quadratic {
//     unimplemented!()
// }
pub fn quadratic_times_linear(
    quad: &Quadratic,
    lin: &Linear,
    env: &Environment,
) -> (Quadratic, Option<HigherOrder>) {
    // Same as linear times quadratic: lin x quad = quad x lin
    linear_times_quadratic(lin, quad, env)
}
pub fn quadratic_times_quadratic(
    lhs: &Quadratic,
    rhs: &Quadratic,
    env: &Environment,
) -> (Quadratic, Option<HigherOrder>) {
    // Computes lhs * rhs.
    // Thus, if lhs is empty, the result is empty.
    // If rhs is empty the result is also empty. As empty is equal to 0.
    if !lhs.has_variables() || !rhs.has_variables() {
        return (Quadratic::empty(lhs.env_id), None);
    }
    // From here on, both terms are not 0. Thus we can do the computations.
    // The multiplication of two quadratic terms can still be a quadratic term in case the variables
    // are equal and binary or spin. If they are different variables a higher order term is generated.
    let mut quadout = Quadratic::empty(lhs.env_id);
    let mut hoout = HigherOrder::empty(lhs.env_id);

    for (lhskey, lhsvalue) in lhs.variables().iter() {
        for (rhskey, rhsvalue) in rhs.variables().iter() {
            let newvalue = lhsvalue * rhsvalue;
            // If the newvalue is 0. Nothing happens, as the result is 0.
            if newvalue == 0.0 {
                continue;
            }
            // The newvalue is not 0. We need to do work.
            let (lhs_a, lhs_b) = Quadratic::get_key_contributions(lhskey);
            let (rhs_a, rhs_b) = Quadratic::get_key_contributions(rhskey);

            let vtype_lhs_a = env.get(&lhs_a).vtype;
            let vtype_lhs_b = env.get(&lhs_b).vtype;
            // let vtype_rhs_a = env.get(&rhs_a).vtype;
            // let vtype_rhs_b = env.get(&rhs_b).vtype;

            let aaeq = lhs_a == rhs_a;
            let abeq = lhs_a == rhs_b;

            let baeq = lhs_b == rhs_a;
            let bbeq = lhs_b == rhs_b;

            // We need to check the combinations one by one. This pattern is
            // coded for maximum performance. Paths are merged where possible
            // in a later step.
            match (aaeq, abeq, baeq, bbeq) {
                (false, false, false, false) => {
                    // No variables match. Can directly create a higher order term.
                    hoout.add_multi(vec![lhs_a, lhs_b, rhs_a, rhs_b], newvalue);
                }
                // lhs_a * lhs_b * rhs_a
                (false, true, false, false) | (false, false, false, true) => {
                    // vvvv IF BINARY OR SPING vvvv
                    // lhs_a == rhs_b:
                    // = lhs_a * lhs_b * rhs_a * rhs_b
                    // = lhs_a * lhs_b * rhs_a * lhs_a
                    // = lhs_a * lhs_a * lhs_b * rhs_a
                    // = lhs_a * lhs_b * rhs_a
                    // ^^^^ IF BINARY OR SPING ^^^^
                    if vtype_lhs_a == Vtype::Binary || vtype_lhs_a == Vtype::Spin {
                        hoout.add_elem(lhs_a, lhs_b, rhs_a, newvalue);
                    } else {
                        hoout.add_multi(vec![lhs_a, lhs_b, rhs_a, rhs_b], newvalue);
                    }
                }
                // (false, false, false, true) => {
                //     // vvvv IF BINARY OR SPING vvvv
                //     // lhs_b == rhs_b:
                //     // = lhs_a * lhs_b * rhs_a * rhs_b
                //     // = lhs_a * lhs_b * rhs_a * lhs_b
                //     // = lhs_a * lhs_b * lhs_b * rhs_a
                //     // = lhs_a * lhs_b * rhs_a
                //     // ^^^^ IF BINARY OR SPING ^^^^
                //     //
                //     // The right side of both keys is equal...
                //     // We need to check the varaible types next, i.e., check if the b variable
                //     // is of type binary or spin. It is sufficient to check a single vtype as
                //     // both variables are equal.
                //     if vtype_lhs_b == Vtype::Binary || vtype_lhs_b == Vtype::Spin {
                //         // We have binary or spin for the right variable.
                //         // We know the left side is not equal to eachother nor to the
                //         // right side. Thus, we know it's a new higher order term with
                //         // lhs_a, rhs_a, and lhs_b
                //         // as lhs_b == rhs_b && lhs_b * lhs_b = lhs_b
                //         hoout.append_elem(lhs_a, rhs_a, lhs_b, newvalue);
                //     } else {
                //         // We have a higher order term with all four variables.
                //         hoout.append_multi(vec![lhs_a, lhs_b, rhs_a, rhs_b], newvalue);
                //     }
                // }
                // lhs_a * lhs_b * rhs_b
                (false, false, true, false) | (true, false, false, false) => {
                    // vvvv IF BINARY OR SPING vvvv
                    // lhs_b == rhs_a
                    // = lhs_a * lhs_b * rhs_a * rhs_b
                    // = lhs_a * lhs_b * lhs_b * rhs_b
                    // = lhs_a * lhs_b * rhs_b
                    // ^^^^ IF BINARY OR SPING ^^^^
                    //
                    // The lhs right side and the rhs left side are equal...
                    // lhs_b == rhs_a
                    // Need to check the variable type next.
                    // sufficient to check type of lhs_b (used already in another path)
                    if vtype_lhs_b == Vtype::Binary || vtype_lhs_b == Vtype::Spin {
                        // We have binary or spin for the lhs_b and rhs_a
                        // thus we have the following variables.
                        // lhs_a, lhs_b, rhs_b
                        // as lhs_b == rhs_a && lhs_b * lhs_b = lhs_b
                        hoout.add_elem(lhs_a, lhs_b, rhs_b, newvalue);
                    } else {
                        // We have a higher order term with all four variables.
                        hoout.add_multi(vec![lhs_a, lhs_b, rhs_a, rhs_b], newvalue);
                    }
                }
                // (true, false, false, false) => {
                //     // vvvv IF BINARY OR SPING vvvv
                //     // lhs_a = rhs_a:
                //     // = lhs_a * lhs_b * rhs_a * rhs_b
                //     // = lhs_a * lhs_b * lhs_a * rhs_b
                //     // = lhs_a * lhs_b * rhs_b
                //     // ^^^^ IF BINARY OR SPING ^^^^
                // }
                // lhs_a * lhs_b
                (false, false, true, true)
                | (true, false, false, true)
                | (true, true, true, true)
                | (true, true, false, false) => {
                    // vvvv IF BINARY OR SPING vvvv
                    // lhs_b = rhs_a
                    // lhs_b = rhs_b
                    // BUT
                    // lhs_a != rhs_a
                    // lhs_a != rhs_b
                    // FINE
                    //
                    // lhs_b == rhs_a & lhs_b = rhs_b --> lhs_b == rhs_a == rhs_b
                    // = lhs_a * lhs_b * rhs_a * rhs_b
                    // = lhs_a * lhs_b * lhs_b * lhs_b
                    // = lhs_a * lhs_b
                    // ^^^^ IF BINARY OR SPING ^^^^

                    // lhs_b == rhs_a && lhs_b == rhs_b
                    // Only lhs_a is different
                    // If binary or spin quadratic...
                    // else higher order with all four.
                    if vtype_lhs_b == Vtype::Binary || vtype_lhs_b == Vtype::Spin {
                        // lhs_b == rhs_a == rhs_b
                        // thus quadratic part with lhs_a and lhs_b
                        quadout.add_elem(&lhs_a, &lhs_b, newvalue);
                    } else {
                        // We have a higher order term with all four variables.
                        hoout.add_multi(vec![lhs_a, lhs_b, rhs_a, rhs_b], newvalue);
                    }
                }
                // (true, false, false, true) => {
                //     // vvvv IF BINARY OR SPING vvvv
                //     // lhs_a = rhs_a:
                //     // lhs_b = rhs_b:
                //     // = lhs_a * lhs_b * rhs_a * rhs_b
                //     // = lhs_a * lhs_b
                //     // ^^^^ IF BINARY OR SPING ^^^^
                // }
                // (true, true, true, true) => {
                //     // vvvv IF BINARY OR SPING vvvv
                //     // All are equal
                //     // = lhs_a * lhs_b * rhs_a * rhs_b
                //     // = lhs_a * lhs_b
                //     // ^^^^ IF BINARY OR SPING ^^^^
                // }
                // (true, true, false, false) => {
                //     // vvvv IF BINARY OR SPING vvvv
                //     // lhs_a = rhs_a
                //     // lhs_a = rhs_b
                //     // BUT
                //     // lhs_b != rhs_a
                //     // lhs_b != rhs_b
                //     // FINE
                //     // = lhs_a * lhs_b * rhs_a * rhs_b
                //     // = lhs_a * lhs_b
                //     // ^^^^ IF BINARY OR SPING ^^^^
                // }
                // lhs_a * rhs_a
                (false, true, false, true) => {
                    // vvvv IF BINARY OR SPING vvvv
                    // lhs_a = rhs_b
                    // lhs_b = rhs_b
                    // BUT
                    // lhs_a != rhs_a
                    // lhs_b != rhs_a
                    // FINE
                    // lhs_a == rhs_b & lhs_b == rhs_b --> lhs_a == lhs_b == rhs_b:
                    // = lhs_a * lhs_b * rhs_a * rhs_b
                    // = lhs_a * lhs_a * rhs_a * lhs_a
                    // = lhs_a * lhs_a * lhs_a * rhs_a
                    // = lhs_a * rhs_a
                    // ^^^^ IF BINARY OR SPING ^^^^
                    if vtype_lhs_a == Vtype::Binary || vtype_lhs_a == Vtype::Spin {
                        // thus quadratic part with lhs_a and rhs_a
                        quadout.add_elem(&lhs_a, &rhs_a, newvalue);
                    } else {
                        // We have a higher order term with all four variables.
                        hoout.add_multi(vec![lhs_a, lhs_b, rhs_a, rhs_b], newvalue);
                    }
                }
                // lhs_a * rhs_b
                (true, false, true, false) => {
                    // vvvv IF BINARY OR SPING vvvv
                    // lhs_a = rhs_a
                    // lhs_b = rhs_a
                    // lhs_a = rhs_a = lhs_b
                    // = lhs_a * lhs_b * rhs_a * rhs_b
                    // = lhs_a * rhs_b
                    // ^^^^ IF BINARY OR SPING ^^^^
                    if vtype_lhs_a == Vtype::Binary || vtype_lhs_a == Vtype::Spin {
                        // thus quadratic part with lhs_a and rhs_b
                        quadout.add_elem(&lhs_a, &rhs_b, newvalue);
                    } else {
                        // We have a higher order term with all four variables.
                        hoout.add_multi(vec![lhs_a, lhs_b, rhs_a, rhs_b], newvalue);
                    }
                }
                (false, true, true, false) => {
                    // CONTRADICTION
                    // vvvv IF BINARY OR SPING vvvv
                    // lhs_a = rhs_b
                    // lhs_b = rhs_a
                    // BUT
                    // lhs_a != rhs_a
                    // lhs_b != rhs_b
                    // lhs_a = rhs_b & lhs_b = rhs_a:
                    // = lhs_a * lhs_b * rhs_a * rhs_b
                    // = lhs_a * lhs_b * lhs_b * lhs_a
                    // = lhs_a * lhs_a * lhs_b * lhs_b
                    // = lhs_a * lhs_b
                    // ^^^^ IF BINARY OR SPING ^^^^
                    panic!("contradiction in equalities... this should have never been possible. The path is '(false, true, true, false)'");
                }
                (false, true, true, true) => {
                    // CONTRADICTION
                    // lhs_b = rhs_a & lhs_b = rhs_b
                    // implies that rhs_a = rhs_b
                    // thus all must be equal... this path can never happen...
                    // should go to the all equal path...
                    //
                    // vvvv IF BINARY OR SPING vvvv
                    // lhs_a = rhs_b &
                    // --> lhs_a = rhs_b = lhs_b =
                    // lhs_a != rhs_a
                    // = lhs_a * lhs_b * rhs_a * rhs_b
                    // ^^^^ IF BINARY OR SPING ^^^^
                    panic!("contradiction in equalities... this should have never been possible. The path is '(false, true, true, true)'");
                }
                (true, false, true, true) => {
                    // CONTRADICTION
                    // vvvv IF BINARY OR SPING vvvv
                    // lhs_a = rhs_a
                    // lhs_b = rhs_a
                    // lhs_b = rhs_b
                    // BUT
                    // lhs_a != rhs_b
                    // CONTRADICTION .... ALL EQUAl
                    // = lhs_a * lhs_b * rhs_a * rhs_b
                    // ^^^^ IF BINARY OR SPING ^^^^
                    panic!("contradiction in equalities... this should have never been possible. The path is '(true, false, true, true)'");
                }
                (true, true, false, true) => {
                    // CONTRADICTION
                    // vvvv IF BINARY OR SPING vvvv
                    // lhs_a = rhs_a
                    // lhs_a = rhs_b
                    // lhs_b = rhs_b
                    // BUT
                    // lhs_b != rhs_a
                    // CONTRADICTION
                    // = lhs_a * lhs_b * rhs_a * rhs_b
                    // ^^^^ IF BINARY OR SPING ^^^^
                    panic!("contradiction in equalities... this should have never been possible. The path is '(true, true, false, true)'");
                }
                (true, true, true, false) => {
                    // CONTRADICTION
                    // vvvv IF BINARY OR SPING vvvv
                    // lhs_a = rhs_a
                    // lhs_a = rhs_b
                    // lhs_b = rhs_a
                    // BUT
                    // lhs_b != rhs_b
                    // CONTRADICTION
                    // = lhs_a * lhs_b * rhs_a * rhs_b
                    // ^^^^ IF BINARY OR SPING ^^^^
                    panic!("contradiction in equalities... this should have never been possible. The path is '(true, true, true, false)'");
                }
            }
        }
    }

    if hoout.has_variables() {
        (quadout, Some(hoout))
    } else {
        (quadout, None)
    }
}
pub fn quadratic_times_higher_order(
    quad: &Quadratic,
    ho: &HigherOrder,
    env: &Environment,
) -> HigherOrder {
    if !quad.has_variables() || !ho.has_variables() {
        return HigherOrder::empty(quad.env_id);
    }

    let mut hoout = HigherOrder::empty(quad.env_id);

    for (quadkey, quadval) in quad.variables().iter() {
        for (hokey, hoval) in ho.variables().iter() {
            let newvalue = quadval * hoval;
            if newvalue == 0.0 {
                // We can skip this contribution safely.
                continue;
            }
            // Need to do some work.
            let (quadcontrib_a, quadcontrib_b) = Quadratic::get_key_contributions(quadkey);
            let hocontribs = HigherOrder::get_key_contributions(hokey.to_string());
            let a_in_ho = hocontribs.contains(&quadcontrib_a);
            let b_in_ho = hocontribs.contains(&quadcontrib_b);

            let vtype_b = env.get(&quadcontrib_b).vtype;
            let vtype_a = env.get(&quadcontrib_a).vtype;

            match (a_in_ho, b_in_ho) {
                (false, false) => {
                    let mut newkeys = vec![quadcontrib_a, quadcontrib_b];
                    newkeys.extend(hocontribs);
                    hoout.add_multi(newkeys, newvalue);
                }
                (false, true) => {
                    // Need to check variable type of `b`
                    if vtype_b == Vtype::Binary || vtype_b == Vtype::Spin {
                        // Just a in the update key.
                        let newkey = HigherOrder::update_key(hokey.to_string(), quadcontrib_a);
                        hoout.add_kv(newkey, newvalue);
                    } else {
                        // Both in the updated key.
                        let mut newkeys = vec![quadcontrib_a, quadcontrib_b];
                        newkeys.extend(hocontribs);
                        hoout.add_multi(newkeys, newvalue);
                    }
                }
                (true, false) => {
                    // Need to check variable type of `a`
                    if vtype_a == Vtype::Binary || vtype_a == Vtype::Spin {
                        // Just b in the update key.
                        let newkey = HigherOrder::update_key(hokey.to_string(), quadcontrib_b);
                        hoout.add_kv(newkey, newvalue);
                    } else {
                        // Both in the updated key.
                        let mut newkeys = vec![quadcontrib_a, quadcontrib_b];
                        newkeys.extend(hocontribs);
                        hoout.add_multi(newkeys, newvalue);
                    }
                }
                (true, true) => {
                    // Need to check variable type of `a` AND `b`
                    let a_bos: bool = vtype_a == Vtype::Binary || vtype_a == Vtype::Spin;
                    let b_bos: bool = vtype_b == Vtype::Binary || vtype_b == Vtype::Spin;

                    match (a_bos, b_bos) {
                        (false, false) => {
                            // Both in the updated key.
                            let mut newkeys = vec![quadcontrib_a, quadcontrib_b];
                            newkeys.extend(hocontribs);
                            hoout.add_multi(newkeys, newvalue);
                        }
                        (false, true) => {
                            // Just a in the updated key.
                            let newkey = HigherOrder::update_key(hokey.to_string(), quadcontrib_a);
                            hoout.add_kv(newkey, newvalue);
                        }
                        (true, false) => {
                            // Just b in the updated key.
                            let newkey = HigherOrder::update_key(hokey.to_string(), quadcontrib_b);
                            hoout.add_kv(newkey, newvalue);
                        }
                        (true, true) => {
                            // None in the updated key.
                            hoout.add_kv(hokey.to_string(), newvalue);
                        }
                    }
                }
            }
        }
    }

    hoout
}

// Higher Order
// pub fn higher_order_times_constant(lhs: &HigherOrder, rhs: &Constant) -> HigherOrder {
//     unimplemented!()
// }
pub fn higher_order_times_linear(ho: &HigherOrder, lin: &Linear, env: &Environment) -> HigherOrder {
    if !ho.has_variables() || !lin.has_variables() {
        return HigherOrder::empty(ho.env_id);
    }
    let mut hoout = HigherOrder::empty(ho.env_id);

    for (linkey, linval) in lin.variables().iter() {
        for (hokey, hoval) in ho.variables().iter() {
            let newvalue = linval * hoval;
            if newvalue == 0.0 {
                // We can skip this contribution safely.
                continue;
            }
            // Need to do some work.
            let hocontribs = HigherOrder::get_key_contributions(hokey.to_string());
            let linkey_in_hokeys = hocontribs.contains(&linkey);

            if linkey_in_hokeys {
                let lin_vtype = env.get(linkey).vtype;
                if lin_vtype == Vtype::Binary || lin_vtype == Vtype::Spin {
                    hoout.add_kv(hokey.to_string(), newvalue);
                } else {
                    let newkey = HigherOrder::update_key(hokey.to_string(), *linkey);
                    hoout.add_kv(newkey, newvalue);
                }
            } else {
                let newkey = HigherOrder::update_key(hokey.to_string(), *linkey);
                hoout.add_kv(newkey, newvalue);
            }
        }
    }

    hoout
}
pub fn higher_order_times_quadratic(
    ho: &HigherOrder,
    quad: &Quadratic,
    env: &Environment,
) -> HigherOrder {
    // ho x quad = quad x ho
    quadratic_times_higher_order(quad, ho, env)
}
pub fn higher_order_times_higher_order(
    lhs: &HigherOrder,
    rhs: &HigherOrder,
    env: &Environment,
) -> HigherOrder {
    if !lhs.has_variables() || !rhs.has_variables() {
        return HigherOrder::empty(lhs.env_id);
    }

    let mut hoout = HigherOrder::empty(lhs.env_id);

    for (lhskey, lhsval) in lhs.variables().iter() {
        for (rhskey, rhsval) in rhs.variables().iter() {
            let newvalue = lhsval * rhsval;
            if newvalue == 0.0 {
                // We can skip this contribution safely.
                continue;
            }
            // Need to do some work.
            let lhskeys = HigherOrder::get_key_contributions(lhskey.to_string());
            let rhskeys = HigherOrder::get_key_contributions(rhskey.to_string());

            let lhs_contrib: Option<Vec<VarId>> = rhskey.get_contained(lhskey.to_string());
            if lhs_contrib.is_none() {
                let mut newkeys = Vec::new();
                newkeys.extend(lhskeys);
                newkeys.extend(rhskeys);
                hoout.add_multi(newkeys, newvalue);
            } else {
                let mut keys_to_add: Vec<VarId> = Vec::new();
                // Only keys that are not binary or spin are added.
                for contrib in lhs_contrib.unwrap().iter() {
                    let vtype = env.get(contrib).vtype;
                    if vtype == Vtype::Binary || vtype == Vtype::Spin {
                        // do nothing; key is not added as remains the key for quadratic.
                        continue;
                    } else {
                        keys_to_add.push(*contrib);
                    }
                }
            }
        }
    }

    hoout
}
