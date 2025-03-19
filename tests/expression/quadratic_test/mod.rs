mod binary_test;
mod integer_test;
mod real_test;
mod spin_test;

use std::rc::Rc;

use aqmodels::core::{
    environment::add_variable,
    operations::{MulAssignToExpression, MulToExpression},
    term::types::{OneVarTerm, OneVarTermConstruction},
    ConcreteBias, ConcreteIndex, Vtype,
};

use crate::common::*;

fn quadratic_expression_base(vtype: Vtype, n: usize) {
    let env = package(create_env::<ConcreteIndex>());
    let biases = random_biases::<ConcreteBias>(n);
    let mut expr = create_linear_expression(Rc::clone(&env), &biases, vtype);

    let multiplier = add_variable(Rc::clone(&env), &"m".to_string(), Some(&vtype), None).unwrap();
    let mscalar = random_bias::<ConcreteBias>();
    expr.mul_assign(&multiplier.mul(mscalar)).unwrap();

    let mut quadratic: Vec<Vec<OneVarTerm<ConcreteIndex, ConcreteBias>>> = biases
        .iter()
        .map(|b| vec![OneVarTerm::new(multiplier.id, b * mscalar)])
        .collect();
    quadratic.push(vec![]);

    assert_eq!(expr.env, env, "envs is wrong");
    assert_eq!(expr.offset, ConcreteBias::default(), "offset is wrong");
    assert_eq!(
        expr.linear.to_vec(),
        &vec![ConcreteBias::default(); biases.len() + 1],
        "linear parts are not equal"
    );
    assert_ne!(
        expr.quadratic, None,
        "quadratic must not be None after multiplications"
    );
    assert_eq!(
        expr.quadratic.unwrap().adj,
        quadratic,
        "the quadratic term is not the expected structure"
    );
    assert_eq!(expr.higher_order, None, "higher order should be None");
    assert_eq!(
        expr.active.len(),
        biases.len() + 1,
        "the number of active variables in the result is false"
    );
    assert_eq!(
        expr.active,
        vec![true; biases.len() + 1],
        "all variables should be active in the result"
    );
    assert_eq!(
        expr.num_variables,
        biases.len() + 1,
        "the number of variables should have increased by one"
    );
}

#[test]
fn quadratic_expression() {
    let n = 100;
    quadratic_expression_base(Vtype::Binary, n);
    quadratic_expression_base(Vtype::Spin, n);
    quadratic_expression_base(Vtype::Integer, n);
    quadratic_expression_base(Vtype::Real, n);
}
