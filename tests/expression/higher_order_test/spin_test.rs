use std::rc::Rc;

use aqmodels::core::{
    operations::{MulAssignToExpression, MulToExpression},
    term::{types::OneVarTerm, HigherOrder},
    ConcreteBias, ConcreteIndex, Vtype,
};

use crate::common::*;

#[test]
fn higher_order_expression_equal_spins_varref() {
    let seed = make_seed();
    let n = 4;

    let env = package(create_env::<ConcreteIndex>());
    let biases = random_biases::<ConcreteBias>(n, seed);
    let (mut expr, vars) =
        create_linear_expression_with_vars(Rc::clone(&env), &biases, Vtype::Spin);

    let multiplier = &vars[0];
    expr.mul_assign(multiplier).unwrap();
    expr.mul_assign(multiplier).unwrap();

    let expected_offset = ConcreteBias::default();

    let mut expected_linear: Vec<ConcreteBias> = biases.clone();
    expected_linear[multiplier.id.0 as usize] = biases[0];

    let expected_quadratic: Vec<Vec<OneVarTerm<ConcreteIndex, ConcreteBias>>> =
        vec![vec![]; biases.len()];
    let expected_higher_order: HigherOrder<ConcreteIndex, ConcreteBias> = HigherOrder::default();

    assert_eq!(expr.env, env, "envs is wrong");
    assert_eq!(expr.offset, expected_offset, "offset is wrong");
    assert_eq!(
        expr.linear.to_vec(),
        &expected_linear,
        "linear parts are not equal"
    );
    assert_ne!(
        expr.quadratic, None,
        "quadratic must not be None after multiplications"
    );
    assert_eq!(
        expr.quadratic.unwrap().adj,
        expected_quadratic,
        "the quadratic term is not the expected structure"
    );
    assert_eq!(
        expr.higher_order,
        Some(expected_higher_order),
        "higher order should be None"
    );
    assert_eq!(
        expr.active.len(),
        biases.len(),
        "the number of active variables in the result is false"
    );
    assert_eq!(
        expr.active,
        vec![true; biases.len()],
        "all variables should be active in the result"
    );
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

    let env = package(create_env::<ConcreteIndex>());
    let biases = random_biases::<ConcreteBias>(n, seed);
    let (mut expr, vars) =
        create_linear_expression_with_vars(Rc::clone(&env), &biases, Vtype::Spin);

    let multiplier = &vars[0];
    expr.mul_assign(&multiplier.mul(biases[0])).unwrap();
    expr.mul_assign(&multiplier.mul(biases[0])).unwrap();

    let expected_offset = ConcreteBias::default();

    let mut expected_linear: Vec<ConcreteBias> =
        biases.iter().map(|b| b * biases[0] * biases[0]).collect();
    expected_linear[multiplier.id.0 as usize] = biases[0] * biases[0] * biases[0];

    let expected_quadratic: Vec<Vec<OneVarTerm<ConcreteIndex, ConcreteBias>>> =
        vec![vec![]; biases.len()];
    let expected_higher_order: HigherOrder<ConcreteIndex, ConcreteBias> = HigherOrder::default();

    assert_eq!(expr.env, env, "envs is wrong");
    assert_eq!(expr.offset, expected_offset, "offset is wrong");
    assert_eq!(
        expr.linear.to_vec(),
        &expected_linear,
        "linear parts are not equal"
    );
    assert_ne!(
        expr.quadratic, None,
        "quadratic must not be None after multiplications"
    );
    assert_eq!(
        expr.quadratic.unwrap().adj,
        expected_quadratic,
        "the quadratic term is not the expected structure"
    );
    assert_eq!(
        expr.higher_order,
        Some(expected_higher_order),
        "higher order should be None"
    );
    assert_eq!(
        expr.active.len(),
        biases.len(),
        "the number of active variables in the result is false"
    );
    assert_eq!(
        expr.active,
        vec![true; biases.len()],
        "all variables should be active in the result"
    );
    assert_eq!(
        expr.num_variables,
        biases.len(),
        "the number of variables should have increased by one"
    );
}
