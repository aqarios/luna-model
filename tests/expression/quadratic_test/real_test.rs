use aqmodels::{
    core::{
        operations::{MulAssignToExpression, MulToExpression},
        term::types::{OneVarTerm, OneVarTermConstruction, TwoVarTerm, TwoVarTermConstruction},
        Vtype,
    },
    types::Bias,
};

use crate::common::*;

#[test]
fn quadratic_expression_equal_real_varref() {
    let seed = make_seed();
    let n = 100;

    let env = create_env();
    let biases = random_biases::<Bias>(n, seed);
    let (mut expr, vars) = create_linear_expression_with_vars(env.clone(), &biases, Vtype::Real);

    let multiplier = &vars[0];
    expr.mul_assign(multiplier).unwrap();

    let expected_linear: Vec<Bias> = vec![Bias::default(); biases.len()];
    let expected_quadratic: Vec<TwoVarTerm> = vec![TwoVarTerm::new(
        0.into(),
        biases
            .iter()
            .enumerate()
            .map(|(i, b)| OneVarTerm::new((i).into(), *b))
            .collect(),
    )];

    assert_eq!(expr.env, env, "envs is wrong");
    assert_eq!(expr.offset, Bias::default(), "offset is wrong");
    assert_eq!(
        expr.linear.to_vec(expr.num_variables),
        expected_linear,
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
fn quadratic_expression_equal_real_expr() {
    let seed = make_seed();
    let n = 100;

    let env = create_env();
    let biases = random_biases::<Bias>(n, seed);
    let (mut expr, vars) = create_linear_expression_with_vars(env.clone(), &biases, Vtype::Real);

    let multiplier = &vars[0];
    expr.mul_assign(&multiplier.mul(1.0)).unwrap();

    let expected_linear: Vec<Bias> = vec![Bias::default(); biases.len()];
    let expected_quadratic: Vec<TwoVarTerm> = vec![TwoVarTerm::new(
        0.into(),
        biases
            .iter()
            .enumerate()
            .map(|(i, b)| OneVarTerm::new((i).into(), *b))
            .collect(),
    )];

    assert_eq!(expr.env, env, "envs is wrong");
    assert_eq!(expr.offset, Bias::default(), "offset is wrong");
    assert_eq!(
        expr.linear.to_vec(expr.num_variables),
        expected_linear,
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
