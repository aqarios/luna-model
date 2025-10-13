use aqmodels::core::{Comparator, Constraint, Constraints, Vtype};

mod common;
use common::*;

#[test]
fn linear_constraint_eq() {
    let seed = make_seed();
    let env = create_env();
    let biases = random_biases(2, seed);
    let expr = create_linear_expression(env, &biases, Vtype::Binary);
    let rhs = random_bias(seed);

    let constr = Constraint::new(expr, rhs, Comparator::Eq, Some("constr".to_string())).unwrap();
    assert_eq!(constr.lhs.offset, 0.0);
    assert_eq!(constr.lhs.linear.to_vec(biases.len()), biases);
    assert_eq!(constr.lhs.quadratic, None);
    assert_eq!(constr.lhs.higher_order, None);
    assert_eq!(constr.rhs, rhs);
    assert_eq!(constr.comparator, Comparator::Eq);
}

#[test]
fn linear_constraint_le() {
    let seed = make_seed();
    let env = create_env();
    let biases = random_biases(2, seed);
    let expr = create_linear_expression(env, &biases, Vtype::Binary);
    let rhs = random_bias(seed);

    let constr = Constraint::new(expr, rhs, Comparator::Le, None).unwrap();
    assert_eq!(constr.lhs.offset, 0.0);
    assert_eq!(constr.lhs.linear.to_vec(biases.len()), biases);
    assert_eq!(constr.lhs.quadratic, None);
    assert_eq!(constr.lhs.higher_order, None);
    assert_eq!(constr.rhs, rhs);
    assert_eq!(constr.comparator, Comparator::Le);
}

#[test]
fn linear_constraint_ge() {
    let seed = make_seed();
    let env = create_env();
    let biases = random_biases(2, seed);
    let expr = create_linear_expression(env, &biases, Vtype::Binary);
    let rhs = random_bias(seed);

    let constr = Constraint::new(expr, rhs, Comparator::Ge, Some("constr".to_string())).unwrap();
    assert_eq!(constr.lhs.offset, 0.0);
    assert_eq!(constr.lhs.linear.to_vec(biases.len()), biases);
    assert_eq!(constr.lhs.quadratic, None);
    assert_eq!(constr.lhs.higher_order, None);
    assert_eq!(constr.rhs, rhs);
    assert_eq!(constr.comparator, Comparator::Ge);
}

#[test]
fn linear_constraints() {
    let seed = make_seed();
    let env = create_env();
    let expr = create_linear_expression(env.clone(), &random_biases(2, seed), Vtype::Binary);
    let rhs = random_bias(seed);

    let constr_a = Constraint::new(expr.clone(), rhs, Comparator::Le, None).unwrap();
    let constr_b = Constraint::new(
        expr.clone(),
        rhs,
        Comparator::Eq,
        Some("constr".to_string()),
    )
    .unwrap();
    let constr_c = Constraint::new(expr.clone(), rhs, Comparator::Ge, None).unwrap();
    let original_constraints = vec![&constr_a, &constr_b, &constr_c];

    let mut constrs = Constraints::default();
    assert_noerror(constrs.add_assign(&constr_a));
    assert_noerror(constrs.add_assign(&constr_b));
    assert_noerror(constrs.add_assign(&constr_c));


    for ((_, constr), actual) in constrs.iter().zip(&original_constraints) {
        assert_eq!(constr.lhs.offset, 0.0);
        assert_eq!(
            constr.lhs.linear.to_vec(constr.lhs.num_variables),
            actual.lhs.linear.to_vec(actual.lhs.num_variables)
        );
        assert_eq!(constr.lhs.quadratic, actual.lhs.quadratic);
        assert_eq!(constr.lhs.higher_order, actual.lhs.higher_order);
        assert_eq!(constr.rhs, actual.rhs);
        assert_eq!(constr.comparator, actual.comparator);
    }
}
