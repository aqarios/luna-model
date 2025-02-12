use std::ops::AddAssign;

use crate::core::higher_order_operations::TermC;
use crate::core::operations::Term;
use crate::core::term::HigherOrder;
use crate::core::Vtype;
use crate::core::{term::Quadratic, Environment, Expression};

use super::constant::constant_times_term;
use super::linear::linear_times_quadratic;

pub fn quadratic_times_expression(
    quadratic: &Quadratic,
    other: &Expression,
    env: &Environment,
    result: &mut Expression,
) {
    // We can use the constant x quadratic to compute quadratic x constant.
    constant_times_term(&other.constant, quadratic, &mut result.quadratic);
    // We can use the linear x quadratic to compute quadratic x linear.
    linear_times_quadratic(&other.linear, quadratic, env, result);
    quadratic_times_quadratic(quadratic, &other.quadratic, env, result);
    quadratic_times_higher_order(quadratic, &other.higher_order, env, result);
}

fn quadratic_times_quadratic(
    lhs: &Quadratic,
    rhs: &Quadratic,
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
        let (a, b) = Quadratic::get_key_contributions(lhskey);

        for (rhskey, rhsval) in rhs.variables().iter() {
            let newvalue = lhsval * rhsval;
            if newvalue == 0.0 {
                // If the newvalue is 0 we can skip the combination.
                continue;
            }

            let (c, d) = Quadratic::get_key_contributions(rhskey);
            let (vtype_a, vtype_b) = (env.get(&a).vtype, env.get(&b).vtype);
            let vtype_c = env.get(&c).vtype;
            // What are the possbile allowed combinations? And which result in the same component?
            // Overall we have la * lb * ra * rb
            // which simplifies in certain cases:
            //
            // If la = lb we know it cannot be of type Binary or Spin.
            // As this results in a linear term or a constant.
            // The same holds for ra = rb.
            // This cannot happen, ever.
            // Let's make sure...
            if (a == b && (vtype_a == Vtype::Binary || vtype_a == Vtype::Spin))
                || (c == d && (vtype_c == Vtype::Binary || vtype_c == Vtype::Spin))
            {
                panic!("one of the quadratic components contains the same variable with type binary or spin")
            }

            // Term is ab * cd
            let a_in_rhs = a == c || a == d;
            let b_in_rhs = b == c || b == d;
            match (vtype_a, vtype_b) {
                (Vtype::Binary, Vtype::Binary) => {
                    // a and b are both binary variables.
                    // If a is equal to c or d we have a higher order term with b, c, d
                    // If b is equal to c or d we have a higher order term with a, c, d
                    // If the top both are true we have a quadratic term with c,d OR a,b
                    // If neither are equal we have a higher order term with a,b,c
                    if a_in_rhs && b_in_rhs {
                        // a = c | a = d & b = c | b = d;
                        result.quadratic.add_elem(a, b, newvalue);
                    } else if a_in_rhs {
                        // the `&& !b_in_rhs` is ensured by the first arm
                        // If a is equal to c or d we have a higher order term with b, c, d
                        result.higher_order.add_elem(b, c, d, newvalue);
                    } else if b_in_rhs {
                        // the `&& !a_in_rhs` is ensured by the first arm
                        // If b is equal to c or d we have a higher order term with a, c, d
                        result.higher_order.add_elem(a, c, d, newvalue);
                    } else {
                        result.higher_order.add_multi(vec![a, b, c, d], newvalue);
                    }
                }
                (Vtype::Spin, Vtype::Spin) => {
                    // a and b are both spin variables.
                    // If there are matching keys it might result in a quadratic component.
                    if a_in_rhs && b_in_rhs {
                        // We multiply multiple equal spin variables with eachother.
                        // This results in a constant as regardless which matching we have
                        // a to c or a to d (equal for b) we know that the product of two
                        // equal spin variables results in +1. Therefore the entire term
                        // results in +1. So we can add the newvalue to the constant offset.
                        result.constant.add_assign(newvalue);
                    } else if a_in_rhs {
                        // the `&& !b_in_rhs` is ensured by the first arm
                        // So a is either equal to c or to d.
                        // We need to know which one. if a = c then we result in a quadratic term
                        // with just b and d. If a = d we get a quadratic component with b and c.
                        if a == c {
                            // a * b * c * d = b * d
                            result.quadratic.add_elem(b, d, newvalue);
                        } else {
                            // a == d
                            // a * b * c * d = b * c
                            result.quadratic.add_elem(b, c, newvalue);
                        }
                    } else if b_in_rhs {
                        // the `&& !a_in_rhs` is ensured by the first arm
                        // Same thing as for a_in_rhs
                        if b == c {
                            // a * b * c * d = a * d
                            result.quadratic.add_elem(a, d, newvalue);
                        } else {
                            // b == d
                            // a * b * c * d = a * c
                            result.quadratic.add_elem(a, c, newvalue);
                        }
                    } else {
                        // Four different variables of type spin and thus we have a new higher
                        // order component.
                        result.higher_order.add_multi(vec![a, b, c, d], newvalue);
                    }
                }
                (Vtype::Binary, Vtype::Spin) => {
                    // ------
                    // a is binary, b is spin
                    // ------
                    binary_spin_helper_quadratic(a_in_rhs, b_in_rhs, a, b, c, d, newvalue, result);
                }
                (Vtype::Spin, Vtype::Binary) => {
                    // ------
                    // a is spin, b is binary
                    // ------
                    binary_spin_helper_quadratic(b_in_rhs, a_in_rhs, b, a, c, d, newvalue, result);
                }
                (_, _) => {
                    // Neither a nor b are spin variables, thus we can directly create the
                    // higher order component with all four variables.
                    result.higher_order.add_multi(vec![a, b, c, d], newvalue);
                }
            }
        }
    }
}

pub fn quadratic_times_higher_order(
    quad: &Quadratic,
    ho: &HigherOrder,
    env: &Environment,
    result: &mut Expression,
) {
    if !quad.has_variables() || !ho.has_variables() {
        // Any of the two elements of the product does not have variables and is thus empty and
        // treated as 0. Therefore, we have no result and do not need to edit the result
        // expression.
        return;
    }

    for (quadkey, quadval) in quad.variables().iter() {
        let (a, b) = Quadratic::get_key_contributions(quadkey);
        for (hokey, hoval) in ho.variables().iter() {
            let newvalue = quadval * hoval;
            if newvalue == 0.0 {
                // If the newvalue is 0 we can skip the combination.
                continue;
            }

            let hocontribs = HigherOrder::get_key_contributions(hokey.clone());
            let (vtype_a, vtype_b) = (env.get(&a).vtype, env.get(&b).vtype);

            let a_in_ho = hocontribs.contains(&a);
            let b_in_ho = hocontribs.contains(&b);
            match (vtype_a, vtype_b) {
                (Vtype::Binary, Vtype::Binary) => {
                    if a_in_ho && b_in_ho {
                        // Both binary variables are contained.
                        // Thus nothing changes for the current higher order component except for
                        // the value of it, i.e., we have a higher order entry with the new value.
                        result.higher_order.add_kv(hokey.clone(), newvalue);
                    } else if a_in_ho {
                        // b not in the current higher order, needs to be added.
                        // We have a new higher order component with the addition of variable b.
                        let newkey = HigherOrder::update_key(hokey.clone(), b);
                        result.higher_order.add_kv(newkey, newvalue);
                    } else if b_in_ho {
                        // a not in the current higher order, needs to be added.
                        let newkey = HigherOrder::update_key(hokey.clone(), a);
                        result.higher_order.add_kv(newkey, newvalue);
                    } else {
                        // No matching variables. Higher order component with all variables in
                        // current higher order and the two from the current quadratic component
                        let mut allvarkeys = vec![a, b];
                        allvarkeys.extend(hocontribs);
                        result.higher_order.add_multi(allvarkeys, newvalue);
                    }
                }
                (Vtype::Binary, Vtype::Spin) => {
                    // a is binary, b is spin
                    binary_spin_helper_higher_order(
                        a_in_ho,
                        b_in_ho,
                        a,
                        b,
                        hokey.clone(),
                        hocontribs,
                        newvalue,
                        result,
                    );
                }
                (Vtype::Spin, Vtype::Binary) => {
                    // a is spin, b is binary
                    binary_spin_helper_higher_order(
                        b_in_ho,
                        a_in_ho,
                        b,
                        a,
                        hokey.clone(),
                        hocontribs,
                        newvalue,
                        result,
                    );
                }
                (Vtype::Spin, Vtype::Spin) => {
                    // this can reduce the degree of the interaction, maybe even to linear if both
                    // variables are contained... Let's see...
                    if a_in_ho && b_in_ho {
                        // both spin variables are contained. We can directly remove both variables
                        // from the higher order variables. and depending of the number of
                        // remaining variables create the respective component in the result
                        // expression.
                        let subset = hocontribs
                            .into_iter()
                            .filter(|&e| !(e == a || e == b)) // keep element if it is not equal
                            // to a or b
                            .collect::<Vec<u32>>();
                        add_subset_helper(subset, newvalue, result);
                    } else if a_in_ho {
                        // remove a, add b
                        let mut subset = hocontribs
                            .into_iter()
                            .filter(|&e| e != a)
                            .collect::<Vec<u32>>();
                        subset.push(b);
                        add_subset_helper(subset, newvalue, result);
                    } else if b_in_ho {
                        // remove b, add a
                        let mut subset = hocontribs
                            .into_iter()
                            .filter(|&e| e != b)
                            .collect::<Vec<u32>>();
                        subset.push(a);
                        add_subset_helper(subset, newvalue, result);
                    } else {
                        // No matching variables. Higher order component with all variables in
                        // current higher order and the two from the current quadratic component
                        let mut allvarkeys = vec![a, b];
                        allvarkeys.extend(hocontribs);
                        result.higher_order.add_multi(allvarkeys, newvalue);
                    }
                }
                (_, _) => {
                    // No matching variables of type binary or spin. Higher order component with all
                    // variables in current higher order and the two from the current quadratic component
                    let mut allvarkeys = vec![a, b];
                    allvarkeys.extend(hocontribs);
                    result.higher_order.add_multi(allvarkeys, newvalue);
                }
            }
        }
    }
}

fn binary_spin_helper_quadratic(
    binary_in_rhs: bool,
    spin_in_rhs: bool,
    binary: u32,
    spin: u32,
    c: u32,
    d: u32,
    newvalue: f64,
    result: &mut Expression,
) {
    let a = binary;
    let b = spin;
    if binary_in_rhs && spin_in_rhs {
        // `binary` and `spin` are both in rhs. Thus we need to know which other variable matches
        // the `binary` and which one matches the `spin`.
        // which one matches `b`.
        // ------
        // a is binary, b is spin
        // ------
        // If a matches c (a = c), b matches d, i.e., a = c AND b = d
        // Therefore:
        // a * b * c * d = a * c * b * d = a * b * d = a * 1
        // The b * d results in +1 as two equal spins always result in a +1
        // The a * c results in a as two equal binaries always result in
        // itself. Thus we have a linear term with just a.
        // ------
        // If a matches d (a = d), b matches c, i.e., a = d AND b = c
        // Therefore:
        // a * b * c * d = a * d * b * c = a * d * 1 = a * 1
        // The b * c results in +1 as two equal spins always result in a +1
        // The a * d results in a as two equal binaries always result in
        // itself. Thus we have a linear term with just a.
        result.linear.add_elem(a, newvalue);
    } else if binary_in_rhs {
        // b is not in rhs. a is binary
        // ------
        // If a = c: a * b * c * d = a * c * b * d = a * b * d
        // as a * c = a (both binary)
        // ------
        // If a = d: a * b * c * d = a * d * b * c = a * b * c
        // as a * d = a (both binary)
        if a == c {
            // a = c
            result.higher_order.add_elem(a, b, d, newvalue);
        } else {
            // a = d
            result.higher_order.add_elem(a, b, c, newvalue);
        }
    } else if spin_in_rhs {
        // a is not in rhs. b is spin
        // ------
        // If b = c: a * b * c * d = a * +1 * d = a * d
        // as b * c = +1 (both equal spin)
        // ------
        // If b = d: a * b * c * d = a * c * b * d = a * c * +1 = a * c
        // as b * d = +1 (both equal spin)
        if b == c {
            result.quadratic.add_elem(a, d, newvalue);
        } else {
            // b == d
            result.quadratic.add_elem(a, c, newvalue);
        }
    } else {
        // Neither a nor b are equal to c or d.
        // Thus we have a degree four higher order component.
        result.higher_order.add_multi(vec![a, b, c, d], newvalue);
    }
}

fn binary_spin_helper_higher_order(
    binary_in_ho: bool,
    spin_in_ho: bool,
    binary: u32,
    spin: u32,
    hokey: String,
    hokeys: Vec<u32>,
    newvalue: f64,
    result: &mut Expression,
) {
    if binary_in_ho && spin_in_ho {
        // binary and spin variable are in the higher order component
        // The binary variable can be safely ignored.
        // However, we need to remove the spin variable from the higher order keys.
        // This can possibly result in a quadratic term. Spcifically, when the current
        // number of variables in the higher order component is three. Then one variable is removed
        // from the interaction, leaving two variables in the interaction thus it being quadratic.
        let subset = hokeys
            .into_iter()
            .filter(|&e| e != spin)
            .collect::<Vec<_>>();
        add_subset_helper(subset, newvalue, result);
    } else if binary_in_ho {
        // spin is not in the higher order component.
        // The binary variable can be safely ignored.
        // A new higher order component is generated with the spin variable added to the current
        // variables.
        let newkey = HigherOrder::update_key(hokey, spin);
        result.higher_order.add_kv(newkey, newvalue);
    } else if spin_in_ho {
        // binary is not in the higher order component.
        // The spin cancels out with the other equal variable, i.e., the entire higher order term
        // is reduced by one variable, but another one is added, i.e. it remains higher order of
        // same degree.
        // First we need to filter out the equal spin variable, as they result in +1.
        let mut subset = hokeys
            .into_iter()
            .filter(|&e| e != spin)
            .collect::<Vec<u32>>();
        // then we add the binary variable, which is not yet contained.
        subset.push(binary);
        add_subset_helper(subset, newvalue, result);
    } else {
        // neither spin nor binary are in the higher order component, thus
        // no matching variables. Higher order component with all variables in
        // current higher order and the two from the current quadratic component
        let mut allvarkeys = vec![binary, spin];
        allvarkeys.extend(hokeys);
        result.higher_order.add_multi(allvarkeys, newvalue);
    }
}

fn add_subset_helper(subset: Vec<u32>, value: f64, result: &mut Expression) {
    match subset.len() {
        // not possible but who knows...
        0 => result.constant.add_assign(value),
        // yep, just a single variable left
        1 => result.linear.add_elem(subset[0], value),
        // two variables left? That sounds quadratic!
        2 => result.quadratic.add_elem(subset[0], subset[1], value),
        // well, back to the higher order stuff :(
        _ => result.higher_order.add_multi(subset, value),
    }
}
