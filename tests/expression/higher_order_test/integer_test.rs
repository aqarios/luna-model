use aqmodels::{
    core::{
        operations::{MulAssignToExpression, MulToExpression},
        term::HigherOrder,
        Vtype,
    },
    types::Bias,
};
use hashbrown::HashMap;

use crate::common::*;

#[test]
fn higher_order_expression_equal_integer_varref() {
    let seed = make_seed();
    let n = 100;

    let env = create_env();
    let biases = random_biases::<Bias>(n, seed);
    let (mut expr, vars) = create_linear_expression_with_vars(env.clone(), &biases, Vtype::Integer);

    let multiplier = &vars[0];
    expr.mul_assign(multiplier).unwrap();
    expr.mul_assign(multiplier).unwrap();

    let expected_linear: Vec<Bias> = vec![Bias::default(); biases.len()];

    let mut expected_higher_order: HashMap<String, Bias> = HashMap::with_capacity(biases.len());
    for (var, bias) in vars.iter().zip(&biases) {
        let key = HigherOrder::make_key(&vec![var.id, multiplier.id, multiplier.id]);
        expected_higher_order.insert(key, *bias);
    }

    assert_eq!(expr.env, env, "envs is wrong");
    assert_eq!(expr.offset, Bias::default(), "offset is wrong");
    assert_eq!(
        expr.linear.to_vec(expr.num_variables),
        expected_linear,
        "linear parts are not equal"
    );
    assert_eq!(
        expr.quadratic, None,
        "quadratic must not be None after multiplications"
    );
    assert_ne!(expr.higher_order, None, "higher order should NOT be None");

    assert_eq!(
        expr.higher_order.as_ref().unwrap().biases,
        expected_higher_order,
        "higher order is incorrect"
    );
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
fn higher_order_expression_equal_integer_expr() {
    let seed = make_seed();
    let n = 100;

    let env = create_env();
    let biases = random_biases::<Bias>(n, seed);
    let (mut expr, vars) = create_linear_expression_with_vars(env.clone(), &biases, Vtype::Integer);

    let multiplier = &vars[0];
    expr.mul_assign(&multiplier.mul(biases[0])).unwrap();
    expr.mul_assign(&multiplier.mul(biases[0])).unwrap();

    let expected_linear: Vec<Bias> = vec![Bias::default(); biases.len()];

    let mut expected_higher_order: HashMap<String, Bias> = HashMap::with_capacity(biases.len());
    for (var, bias) in vars.iter().zip(&biases) {
        let key = HigherOrder::make_key(&vec![var.id, multiplier.id, multiplier.id]);
        expected_higher_order.insert(key, *bias * biases[0] * biases[0]);
    }

    assert_eq!(expr.env, env, "envs is wrong");
    assert_eq!(expr.offset, Bias::default(), "offset is wrong");
    assert_eq!(
        expr.linear.to_vec(expr.num_variables),
        expected_linear,
        "linear parts are not equal"
    );
    assert_eq!(
        expr.quadratic, None,
        "quadratic must be None after multiplications if it would be empty."
    );
    assert_eq!(
        expr.higher_order.as_ref().unwrap().biases,
        expected_higher_order,
        "higher order does not have expected values"
    );
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
