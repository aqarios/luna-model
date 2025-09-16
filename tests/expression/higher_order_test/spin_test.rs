use aqmodels::{
    core::{
        operations::{MulAssignToExpression, MulToExpression},
        Vtype,
    },
    types::Bias,
};

use crate::common::*;

#[test]
fn higher_order_expression_equal_spins_varref() {
    let seed = make_seed();
    let n = 4;

    let env = create_env();
    let biases = random_biases::<Bias>(n, seed);
    let (mut expr, vars) = create_linear_expression_with_vars(env.clone(), &biases, Vtype::Spin);

    let multiplier = &vars[0];
    expr.mul_assign(multiplier).unwrap();
    expr.mul_assign(multiplier).unwrap();

    let expected_offset = Bias::default();

    let mut expected_linear: Vec<Bias> = biases.clone();
    expected_linear[multiplier.id.0 as usize] = biases[0];

    assert_eq!(expr.env, env, "envs is wrong");
    assert_eq!(expr.offset, expected_offset, "offset is wrong");
    assert_eq!(
        expr.linear.to_vec(expr.num_variables),
        expected_linear,
        "linear parts are not equal"
    );
    assert_eq!(
        expr.quadratic, None,
        "quadratic must be None after multiplications if result would be empty quadratic expression."
    );
    assert_eq!(expr.higher_order, None, "higher order should be None");
    // assert_eq!(
    //     expr.active.len(),
    //     biases.len(),
    //     "the number of active variables in the result is false"
    // );
    // assert_eq!(
    //     expr.active,
    //     vec![true; biases.len()],
    //     "all variables should be active in the result"
    // );
    assert_eq!(
        expr.num_variables,
        biases.len(),
        "the number of variables should have increased by one"
    );
}

#[test]
fn higher_order_expression_equal_spins_expr() {
    let seed = make_seed();
    let n = 100;

    let env = create_env();
    let biases = random_biases::<Bias>(n, seed);
    let (mut expr, vars) = create_linear_expression_with_vars(env.clone(), &biases, Vtype::Spin);

    let multiplier = &vars[0];
    expr.mul_assign(&multiplier.mul(biases[0])).unwrap();
    expr.mul_assign(&multiplier.mul(biases[0])).unwrap();

    let expected_offset = Bias::default();

    let mut expected_linear: Vec<Bias> = biases.iter().map(|b| b * biases[0] * biases[0]).collect();
    expected_linear[multiplier.id.0 as usize] = biases[0] * biases[0] * biases[0];

    assert_eq!(expr.env, env, "envs is wrong");
    assert_eq!(expr.offset, expected_offset, "offset is wrong");
    assert_eq!(
        expr.linear.to_vec(expr.num_variables),
        expected_linear,
        "linear parts are not equal"
    );
    assert_eq!(
        expr.quadratic, None,
        "quadratic must not be None after multiplications"
    );
    assert_eq!(expr.higher_order, None, "higher order should be None");
    // assert_eq!(
    //     expr.active.len(),
    //     biases.len(),
    //     "the number of active variables in the result is false"
    // );
    // assert_eq!(
    //     expr.active,
    //     vec![true; biases.len()],
    //     "all variables should be active in the result"
    // );
    assert_eq!(
        expr.num_variables,
        biases.len(),
        "the number of variables should have increased by one"
    );
}
