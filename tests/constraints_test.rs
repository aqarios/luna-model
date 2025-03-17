use std::{ops::Deref, rc::Rc};

use aqmodels::core::{Comparator, Constraint, Constraints, VarId, Vtype};

mod common;
use common::*;

#[test]
fn linear_constraint_eq() {
    let env = package(create_env::<VarId>());
    let biases = random_biases::<f64>(2);
    let expr = package(create_linear_expression(env, &biases, Vtype::Binary));
    let rhs = random_bias();

    let constr = Constraint::new(expr, rhs, Comparator::Eq);
    let binding = constr.lhs.borrow();
    let lhs = binding.deref();

    assert_eq!(lhs.offset, 0.0);
    assert_eq!(lhs.linear.to_vec(), &biases);
    assert_eq!(lhs.quadratic, None);
    assert_eq!(lhs.higher_order, None);
    assert_eq!(constr.rhs, rhs);
    assert_eq!(constr.comparator, Comparator::Eq);
}

#[test]
fn linear_constraint_le() {
    let env = package(create_env::<VarId>());
    let biases = random_biases::<f64>(2);
    let expr = package(create_linear_expression(env, &biases, Vtype::Binary));
    let rhs = random_bias();

    let constr = Constraint::new(expr, rhs, Comparator::Leq);
    let binding = constr.lhs.borrow();
    let lhs = binding.deref();

    assert_eq!(lhs.offset, 0.0);
    assert_eq!(lhs.linear.to_vec(), &biases);
    assert_eq!(lhs.quadratic, None);
    assert_eq!(lhs.higher_order, None);
    assert_eq!(constr.rhs, rhs);
    assert_eq!(constr.comparator, Comparator::Leq);
}

#[test]
fn linear_constraint_ge() {
    let env = package(create_env::<VarId>());
    let biases = random_biases::<f64>(2);
    let expr = package(create_linear_expression(env, &biases, Vtype::Binary));
    let rhs = random_bias();

    let constr = Constraint::new(expr, rhs, Comparator::Geq);
    let binding = constr.lhs.borrow();
    let lhs = binding.deref();

    assert_eq!(lhs.offset, 0.0);
    assert_eq!(lhs.linear.to_vec(), &biases);
    assert_eq!(lhs.quadratic, None);
    assert_eq!(lhs.higher_order, None);
    assert_eq!(constr.rhs, rhs);
    assert_eq!(constr.comparator, Comparator::Geq);
}

#[test]
fn linear_constraints() {
    let env = package(create_env::<VarId>());
    let expr = package(create_linear_expression(
        Rc::clone(&env),
        &random_biases::<f64>(2),
        Vtype::Binary,
    ));
    let rhs = random_bias();

    let constr_a = Constraint::new(Rc::clone(&expr), rhs, Comparator::Leq);
    let constr_b = Constraint::new(Rc::clone(&expr), rhs, Comparator::Eq);
    let constr_c = Constraint::new(Rc::clone(&expr), rhs, Comparator::Geq);
    let original_constraints = vec![&constr_a, &constr_b, &constr_c];

    let mut constrs = Constraints::default();
    constrs += &constr_a;
    constrs += &constr_b;
    constrs += &constr_c;

    for (constr, actual) in constrs.iter().zip(&original_constraints) {
        let constr_binding = constr.lhs.borrow();
        let constr_lhs = constr_binding.deref();

        let actual_binding = actual.lhs.borrow();
        let actual_lhs = actual_binding.deref();

        assert_eq!(constr_lhs.offset, 0.0);
        assert_eq!(constr_lhs.linear.to_vec(), actual_lhs.linear.to_vec());
        assert_eq!(constr_lhs.quadratic, actual_lhs.quadratic);
        assert_eq!(constr_lhs.higher_order, actual_lhs.higher_order);
        assert_eq!(constr.rhs, actual.rhs);
        assert_eq!(constr.comparator, actual.comparator);
    }
}
