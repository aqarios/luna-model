use std::{cell::RefCell, rc::Rc};

use aqmodels::core::{
    environment::add_variable,
    expression::{BiasConstraints, ExpressionBaseCreation, IndexConstraints},
    operations::{AddToExpression, MulToExpression},
    Environment, Expression, VarRef, Vtype,
};
use num::abs;
use rand::{
    distr::{Distribution, StandardUniform},
    rngs::StdRng,
    Rng, RngCore, SeedableRng,
};

pub fn make_seed() -> u64 {
    let seed: u64 = rand::rng().next_u64();
    eprintln!(
        "
**********************************
Random Seed = {}
**********************************
    ",
        seed
    );
    seed
}

pub fn random_bias<B: Default + std::ops::Add<f64, Output = B>>(seed: u64) -> B
where
    StandardUniform: Distribution<B>,
{
    // B::default() + 0.5
    let mut rng = StdRng::seed_from_u64(seed);
    rng.random()
}

pub fn random_biases<B: Copy + Default + std::ops::Add<f64, Output = B>>(
    n: usize,
    seed: u64,
) -> Vec<B>
where
    StandardUniform: Distribution<B>,
{
    // vec![B::default() + 0.5; n]
    let mut rng = StdRng::seed_from_u64(seed);
    (0..n).map(|_| rng.random()).collect()
}

pub fn create_linear_expression_with_vars<I: IndexConstraints, B: BiasConstraints>(
    env: Rc<RefCell<Environment<I>>>,
    biases: &Vec<B>,
    vtype: Vtype,
) -> (Expression<I, B>, Vec<VarRef<I>>) {
    let varname_prefix = match vtype {
        Vtype::Binary => "b",
        Vtype::Spin => "s",
        Vtype::Integer => "i",
        Vtype::Real => "r",
    };
    let vars: Vec<VarRef<I>> = (0..biases.len())
        .map(|i| {
            add_variable(
                Rc::clone(&env),
                &format!("{}{}", varname_prefix, i),
                Some(&vtype),
                None,
            )
            .unwrap()
        })
        .collect();
    let mut expr = Expression::empty(Rc::clone(&env));
    for (v, b) in vars.iter().zip(biases) {
        let tmp = &v.mul(*b);
        expr = expr.add(tmp).unwrap();
    }
    (expr, vars)
}

pub fn create_linear_expression<I: IndexConstraints, B: BiasConstraints>(
    env: Rc<RefCell<Environment<I>>>,
    biases: &Vec<B>,
    vtype: Vtype,
) -> Expression<I, B> {
    create_linear_expression_with_vars(env, biases, vtype).0
}

pub fn create_env<I: IndexConstraints>() -> Environment<I> {
    Environment::new()
}

pub fn package<T>(value: T) -> Rc<RefCell<T>> {
    Rc::new(RefCell::new(value))
}

pub fn almost_equal(a: f64, b: f64, epsilon: Option<f64>, abs_th: Option<f64>) -> bool {
    let epsilon = epsilon.unwrap_or(128_f64 * f64::EPSILON);
    let abs_th = abs_th.unwrap_or(f64::MIN_POSITIVE);

    assert!(f64::EPSILON <= epsilon);
    assert!(epsilon < 1_f64);

    let diff = (a - b).abs();
    let norm = (abs(a) + abs(b)).min(f64::MAX);
    diff < abs_th.max(epsilon * norm)
}
