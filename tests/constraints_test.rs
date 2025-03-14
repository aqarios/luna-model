use std::ops::Deref;

use aqmodels::core::{Comparator, Constraint, VarId, Vtype};

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
