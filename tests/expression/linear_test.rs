use std::rc::Rc;

use aqmodels::core::{ConcreteBias, ConcreteIndex, Vtype};

use crate::common::*;

fn linear_expression_base(vtype: Vtype, n: usize) {
    let env = package(create_env::<ConcreteIndex>());
    let biases = random_biases::<ConcreteBias>(n);
    let expr = create_linear_expression(Rc::clone(&env), &biases, vtype);

    assert_eq!(expr.env, env);
    assert_eq!(expr.offset, ConcreteBias::default());
    assert_eq!(expr.linear.len(), biases.len());
    assert_eq!(expr.linear.to_vec(), &biases);
    assert_eq!(expr.quadratic, None);
    assert_eq!(expr.higher_order, None);
    assert_eq!(expr.active.len(), biases.len());
    assert_eq!(expr.active, vec![true; biases.len()]);
    assert_eq!(expr.num_variables, biases.len());
}

#[test]
fn linear_expression() {
    linear_expression_base(Vtype::Binary, 0);
    linear_expression_base(Vtype::Binary, 100);
    linear_expression_base(Vtype::Spin, 0);
    linear_expression_base(Vtype::Spin, 100);
    linear_expression_base(Vtype::Integer, 0);
    linear_expression_base(Vtype::Integer, 100);
    linear_expression_base(Vtype::Real, 0);
    linear_expression_base(Vtype::Real, 100);
}
