use std::{cell::RefCell, rc::Rc};

use aqmodels::core::{
    environment::add_variable,
    expression::{BiasConstraints, ExpressionBaseCreation, IndexConstraints},
    operations::{AddToExpression, MulToExpression},
    Environment, Expression, VarRef, Vtype,
};
use rand::{
    distr::{Distribution, StandardUniform},
    Rng,
};

pub fn random_bias<B>() -> B
where
    StandardUniform: Distribution<B>,
{
    let mut rng = rand::rng();
    rng.random()
}

pub fn random_biases<B: Copy>(n: usize) -> Vec<B>
where
    StandardUniform: Distribution<B>,
{
    let mut rng = rand::rng();
    (0..n).map(|_| rng.random()).collect()
}

// pub fn create_binary_linear_expression<I: IndexConstraints, B: BiasConstraints>(
//     env: Rc<RefCell<Environment<I>>>,
//     biases: &Vec<B>,
// ) -> Expression<I, B> {
//     create_linear_expression(env, biases, Vtype::Binary)
// }
pub fn create_linear_expression_with_vars<I: IndexConstraints, B: BiasConstraints>(
    env: Rc<RefCell<Environment<I>>>,
    biases: &Vec<B>,
    vtype: Vtype,
) -> (Expression<I, B>, Vec<VarRef<I>>) {
    let vars: Vec<VarRef<I>> = (0..biases.len())
        .map(|i| add_variable(Rc::clone(&env), &format!("b{}", i), Some(&vtype), None).unwrap())
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
