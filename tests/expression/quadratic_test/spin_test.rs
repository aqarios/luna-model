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
fn quadratic_expression_equal_spins_varref() {
    let seed = make_seed();
    let n = 100;

    let env = create_env();
    let biases = random_biases::<Bias>(n, seed);
    let (mut expr, vars) = create_linear_expression_with_vars(env.clone(), &biases, Vtype::Spin);

    let multiplier = &vars[0];
    expr.mul_assign(multiplier).unwrap();

    let expected_offset = biases[0];

    let mut expected_linear: Vec<Bias> = vec![0.0];
    expected_linear.append(&mut vec![Bias::default(); biases.len() - 1]);

    // Here, the creation of the quadratic variable is a bit more tricky.
    // As the smaller value will always contain the interaction.
    // In this case, we multiply with the variable with the smallest index,
    // so we know that all interactions will be located at this position.
    let expected_quadratic_base: Vec<Vec<OneVarTerm>> = vec![biases[1..]
        .iter()
        .enumerate()
        .map(|(i, b)| OneVarTerm::new((i + 1).into(), *b))
        .collect()];
    let expected_quadratic: Vec<TwoVarTerm> = expected_quadratic_base
        .iter()
        .enumerate()
        .map(|(i, vo)| TwoVarTerm::new(i.into(), vo.to_vec()))
        .collect();

    assert_eq!(expr.env, env, "envs is wrong");
    assert_eq!(expr.offset, expected_offset, "offset is wrong");
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
fn quadratic_expression_equal_spins_expr() {
    let seed = make_seed();
    let n = 100;

    let env = create_env();
    let biases = random_biases::<Bias>(n, seed);
    let (mut expr, vars) = create_linear_expression_with_vars(env.clone(), &biases, Vtype::Spin);

    let multiplier = &vars[0];
    expr.mul_assign(&multiplier.mul(1.0)).unwrap();

    let expected_offset = biases[0];

    let mut expected_linear: Vec<Bias> = vec![0.0];
    expected_linear.append(&mut vec![Bias::default(); biases.len() - 1]);

    // Here, the creation of the quadratic variable is a bit more tricky.
    // As the smaller value will always contain the interaction.
    // In this case, we multiply with the variable with the smallest index,
    // so we know that all interactions will be located at this position.
    let expected_quadratic: Vec<TwoVarTerm> = vec![TwoVarTerm::new(
        0.into(),
        biases[1..]
            .iter()
            .enumerate()
            .map(|(i, b)| OneVarTerm::new((i + 1).into(), *b))
            .collect(),
    )];

    assert_eq!(expr.env, env, "envs is wrong");
    assert_eq!(expr.offset, expected_offset, "offset is wrong");
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
