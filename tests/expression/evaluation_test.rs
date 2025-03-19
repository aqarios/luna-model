use std::{ops::Index, rc::Rc};

use aqmodels::core::{
    environment::add_variable,
    expression::ExpressionEvaluation,
    operations::{MulAssignToExpression, MulToExpression},
    ConcreteBias, ConcreteIndex, Vtype,
};

use crate::common::{
    almost_equal, create_env, create_linear_expression, make_seed, package, random_bias,
    random_biases,
};

struct DSample {
    values: Vec<ConcreteBias>,
}

impl DSample {
    fn new(values: Vec<ConcreteBias>) -> Self {
        Self { values }
    }
}

impl Index<ConcreteIndex> for DSample {
    type Output = ConcreteBias;

    fn index(&self, index: ConcreteIndex) -> &Self::Output {
        &self.values[index.0 as usize]
    }
}

fn evaluate_linear_expression(vtype: Vtype, n: usize) {
    let seed = make_seed();
    let env = package(create_env::<ConcreteIndex>());
    let biases = random_biases::<ConcreteBias>(n, seed);
    let expr = create_linear_expression(Rc::clone(&env), &biases, vtype);

    let expected: ConcreteBias = biases.iter().map(|b| b).sum();
    let result = expr.evaluate_sample(&DSample::new(vec![1.0; biases.len()]));

    assert!(
        almost_equal(expected, result, None, None),
        "Evaluation result and expected values differ: {} (is {})",
        expected,
        result
    )
}

fn evaluate_quadratic_expression(vtype: Vtype, n: usize) {
    let seed = make_seed();
    let env = package(create_env::<ConcreteIndex>());
    let biases = random_biases::<ConcreteBias>(n, seed);
    let mut expr = create_linear_expression(Rc::clone(&env), &biases, vtype);

    let multiplier = add_variable(Rc::clone(&env), &"m".to_string(), Some(&vtype), None).unwrap();
    let mscalar = random_bias::<ConcreteBias>(seed);
    expr.mul_assign(&multiplier.mul(mscalar)).unwrap();

    let expected: ConcreteBias = biases.iter().map(|b| b * mscalar).sum::<ConcreteBias>();
    let result = expr.evaluate_sample(&DSample::new(vec![1.0; biases.len() + 1]));

    assert!(
        almost_equal(expected, result, None, None),
        "Evaluation result and expected values differ: {} (is {})",
        expected,
        result
    )
}

fn evaluate_higher_order_expression(vtype: Vtype, n: usize) {
    let seed = make_seed();
    let env = package(create_env::<ConcreteIndex>());
    let biases = random_biases::<ConcreteBias>(n, seed);
    let mut expr = create_linear_expression(Rc::clone(&env), &biases, vtype);

    let ma = add_variable(Rc::clone(&env), &"ma".to_string(), Some(&vtype), None).unwrap();
    let mb = add_variable(Rc::clone(&env), &"mb".to_string(), Some(&vtype), None).unwrap();
    let ma_scalar = random_bias::<ConcreteBias>(seed);
    let mb_scalar = random_bias::<ConcreteBias>(seed);
    expr.mul_assign(&ma.mul(ma_scalar)).unwrap();
    expr.mul_assign(&mb.mul(mb_scalar)).unwrap();

    let expected: ConcreteBias = biases
        .iter()
        .map(|b| b * ma_scalar * mb_scalar)
        .sum::<ConcreteBias>();
    let result = expr.evaluate_sample(&DSample::new(vec![1.0; biases.len() + 2]));

    assert!(
        almost_equal(expected, result, None, None),
        "Evaluation result and expected values differ: {} (is {})",
        expected,
        result
    )
}

#[test]
fn evaluate_linear_expression_test() {
    let n = 100;
    evaluate_linear_expression(Vtype::Binary, n);
    evaluate_linear_expression(Vtype::Spin, n);
    evaluate_linear_expression(Vtype::Integer, n);
    evaluate_linear_expression(Vtype::Real, n);
}

#[test]
fn evaluate_quadratic_expression_test() {
    let n = 100;
    evaluate_quadratic_expression(Vtype::Binary, n);
    evaluate_quadratic_expression(Vtype::Spin, n);
    evaluate_quadratic_expression(Vtype::Integer, n);
    evaluate_quadratic_expression(Vtype::Real, n);
}

#[test]
fn evaluate_higher_order_expression_test() {
    let n = 100;
    evaluate_higher_order_expression(Vtype::Binary, n);
    evaluate_higher_order_expression(Vtype::Spin, n);
    evaluate_higher_order_expression(Vtype::Integer, n);
    evaluate_higher_order_expression(Vtype::Real, n);
}
