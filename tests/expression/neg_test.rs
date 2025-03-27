use std::{ops::Index, rc::Rc};

use aqmodels::core::{
    environment::add_variable,
    expression::ExpressionEvaluation,
    operations::{AddAssignToExpression, AddToExpression, MulAssignToExpression, MulToExpression},
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

fn evaluate_linear_expression_neg(vtype: Vtype, n: usize) {
    let seed = make_seed();
    let env = package(create_env::<ConcreteIndex>());
    let biases = random_biases::<ConcreteBias>(n, seed);
    let base_expr = create_linear_expression(Rc::clone(&env), &biases, vtype);
    let expr = -base_expr;

    let expected: ConcreteBias = biases.iter().map(|b| -b).sum();
    let result = expr.evaluate_sample(&DSample::new(vec![1.0; biases.len()]));

    assert!(
        almost_equal(expected, result, None, None),
        "Evaluation result and expected values differ: {} (is {})",
        expected,
        result
    )
}

fn evaluate_quadratic_expression_neg(vtype: Vtype, n: usize) {
    let seed = make_seed();
    let env = package(create_env::<ConcreteIndex>());
    let biases = random_biases::<ConcreteBias>(n, seed);
    let mut expr = create_linear_expression(Rc::clone(&env), &biases, vtype);

    let multiplier = add_variable(Rc::clone(&env), &"m".to_string(), Some(&vtype), None).unwrap();
    let mscalar = random_bias::<ConcreteBias>(seed);
    expr.mul_assign(&multiplier.mul(mscalar)).unwrap();

    let expr = -expr;

    let expected: ConcreteBias = biases.iter().map(|b| -b * mscalar).sum::<ConcreteBias>();
    let result = expr.evaluate_sample(&DSample::new(vec![1.0; biases.len() + 1]));

    assert!(
        almost_equal(expected, result, None, None),
        "Evaluation result and expected values differ: {} (is {})",
        expected,
        result
    )
}

fn evaluate_higher_order_expression_neg(vtype: Vtype, n: usize) {
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

    let expr = -expr;

    let expected: ConcreteBias = biases
        .iter()
        .map(|b| -b * ma_scalar * mb_scalar)
        .sum::<ConcreteBias>();
    let result = expr.evaluate_sample(&DSample::new(vec![1.0; biases.len() + 2]));

    assert!(
        almost_equal(expected, result, None, None),
        "Evaluation result and expected values differ: {} (is {})",
        expected,
        result
    )
}

fn evaluate_mixed_order_mixed_vtype_expression_neg(n: usize) {
    let seed = make_seed();
    let env = package(create_env::<ConcreteIndex>());

    let binary_biases = random_biases::<ConcreteBias>(n, seed);
    let binary_expr = create_linear_expression(Rc::clone(&env), &binary_biases, Vtype::Binary);
    let spin_biases = random_biases::<ConcreteBias>(n, seed);
    let spin_expr = create_linear_expression(Rc::clone(&env), &binary_biases, Vtype::Spin);
    let int_biases = random_biases::<ConcreteBias>(n, seed);
    let int_expr = create_linear_expression(Rc::clone(&env), &binary_biases, Vtype::Integer);
    let real_biases = random_biases::<ConcreteBias>(n, seed);
    let real_expr = create_linear_expression(Rc::clone(&env), &binary_biases, Vtype::Real);

    let mb = add_variable(
        Rc::clone(&env),
        &"mb".to_string(),
        Some(&Vtype::Binary),
        None,
    )
    .unwrap();
    let ms = add_variable(Rc::clone(&env), &"ms".to_string(), Some(&Vtype::Spin), None).unwrap();
    let mi = add_variable(
        Rc::clone(&env),
        &"mi".to_string(),
        Some(&Vtype::Integer),
        None,
    )
    .unwrap();
    let mr = add_variable(Rc::clone(&env), &"mr".to_string(), Some(&Vtype::Real), None).unwrap();

    let mbsc = random_bias::<ConcreteBias>(seed);
    let mssc = random_bias::<ConcreteBias>(seed);
    let misc = random_bias::<ConcreteBias>(seed);
    let mrsc = random_bias::<ConcreteBias>(seed);

    // Quadratics
    let quad_binary = binary_expr.mul(&mb.mul(mbsc)).unwrap();
    let quad_spin = spin_expr.mul(&ms.mul(mssc)).unwrap();
    let quad_int = int_expr.mul(&mi.mul(misc)).unwrap();
    let quad_real = real_expr.mul(&mr.mul(mrsc)).unwrap();

    // Higher Orders
    let mut ho_binary = binary_expr.mul(&mb.mul(mbsc)).unwrap();
    ho_binary.mul_assign(&ms.mul(mssc)).unwrap();
    ho_binary.mul_assign(&mi.mul(misc)).unwrap();
    ho_binary.mul_assign(&mr.mul(mrsc)).unwrap();
    let mut ho_spin = spin_expr.mul(&mb.mul(mbsc)).unwrap();
    ho_spin.mul_assign(&ms.mul(mssc)).unwrap();
    ho_spin.mul_assign(&mi.mul(misc)).unwrap();
    ho_spin.mul_assign(&mr.mul(mrsc)).unwrap();
    let mut ho_int = int_expr.mul(&mb.mul(mbsc)).unwrap();
    ho_int.mul_assign(&ms.mul(mssc)).unwrap();
    ho_int.mul_assign(&mi.mul(misc)).unwrap();
    ho_int.mul_assign(&mr.mul(mrsc)).unwrap();
    let mut ho_real = real_expr.mul(&mb.mul(mbsc)).unwrap();
    ho_real.mul_assign(&ms.mul(mssc)).unwrap();
    ho_real.mul_assign(&mi.mul(misc)).unwrap();
    ho_real.mul_assign(&mr.mul(mrsc)).unwrap();

    // Linear
    let mut expr = binary_expr.add(&spin_expr).unwrap();
    expr.add_assign(&int_expr).unwrap();
    expr.add_assign(&real_expr).unwrap();
    // Quadratics
    expr.add_assign(&quad_binary).unwrap();
    expr.add_assign(&quad_spin).unwrap();
    expr.add_assign(&quad_int).unwrap();
    expr.add_assign(&quad_real).unwrap();
    // Higher Orders
    expr.add_assign(&ho_binary).unwrap();
    expr.add_assign(&ho_spin).unwrap();
    expr.add_assign(&ho_int).unwrap();
    expr.add_assign(&ho_real).unwrap();

    // eprintln!("Expression =\n\n{:?}", expr);
    let expr = -expr;

    // Expected evaluated value
    let mut expected: ConcreteBias = ConcreteBias::default();
    // Linear sums
    expected += -binary_biases.iter().map(|b| b).sum::<ConcreteBias>();
    expected += -spin_biases.iter().map(|b| b).sum::<ConcreteBias>();
    expected += -int_biases.iter().map(|b| b).sum::<ConcreteBias>();
    expected += -real_biases.iter().map(|b| b).sum::<ConcreteBias>();
    // Quadratic sums
    expected += -binary_biases.iter().map(|b| b * mbsc).sum::<ConcreteBias>();
    expected += -spin_biases.iter().map(|b| b * mssc).sum::<ConcreteBias>();
    expected += -int_biases.iter().map(|b| b * misc).sum::<ConcreteBias>();
    expected += -real_biases.iter().map(|b| b * mrsc).sum::<ConcreteBias>();
    // Higher Order Sums
    expected += -binary_biases
        .iter()
        .map(|b| b * mbsc * mssc * misc * mrsc)
        .sum::<ConcreteBias>();
    expected += -spin_biases
        .iter()
        .map(|b| b * mbsc * mssc * misc * mrsc)
        .sum::<ConcreteBias>();
    expected += -int_biases
        .iter()
        .map(|b| b * mbsc * mssc * misc * mrsc)
        .sum::<ConcreteBias>();
    expected += -real_biases
        .iter()
        .map(|b| b * mbsc * mssc * misc * mrsc)
        .sum::<ConcreteBias>();

    let result = expr.evaluate_sample(&DSample::new(vec![
        1.0;
        binary_biases.len()
            + spin_biases.len()
            + int_biases.len()
            + real_biases.len()
            + 4
    ]));

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
    evaluate_linear_expression_neg(Vtype::Binary, n);
    evaluate_linear_expression_neg(Vtype::Spin, n);
    evaluate_linear_expression_neg(Vtype::Integer, n);
    evaluate_linear_expression_neg(Vtype::Real, n);
}

#[test]
fn evaluate_quadratic_expression_test() {
    let n = 100;
    evaluate_quadratic_expression_neg(Vtype::Binary, n);
    evaluate_quadratic_expression_neg(Vtype::Spin, n);
    evaluate_quadratic_expression_neg(Vtype::Integer, n);
    evaluate_quadratic_expression_neg(Vtype::Real, n);
}

#[test]
fn evaluate_higher_order_expression_test() {
    let n = 100;
    evaluate_higher_order_expression_neg(Vtype::Binary, n);
    evaluate_higher_order_expression_neg(Vtype::Spin, n);
    evaluate_higher_order_expression_neg(Vtype::Integer, n);
    evaluate_higher_order_expression_neg(Vtype::Real, n);
}

#[test]
fn evaluate_mixed_order_mixed_vtype_expression_test() {
    let n = 100;
    evaluate_mixed_order_mixed_vtype_expression_neg(n);
}
