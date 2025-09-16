use aqmodels::{core::Vtype, types::Bias};

use crate::common::*;

fn linear_expression_base(vtype: Vtype, n: usize) {
    let seed = make_seed();
    let env = create_env();
    let biases = random_biases(n, seed);
    let expr = create_linear_expression(env.clone(), &biases, vtype);

    assert_eq!(expr.env, env);
    assert_eq!(expr.offset, Bias::default());
    assert_eq!(expr.linear.len(), biases.len());
    assert_eq!(expr.linear.to_vec(expr.num_variables), biases);
    assert_eq!(expr.quadratic, None);
    assert_eq!(expr.higher_order, None);
    // assert_eq!(expr.active.len(), biases.len());
    // assert_eq!(expr.active, vec![true; biases.len()]);
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
