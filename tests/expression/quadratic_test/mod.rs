mod binary_test;
mod integer_test;
mod real_test;
mod spin_test;

use aqmodels::{
    core::{
        operations::{MulAssignToExpression, MulToExpression},
        term::types::{OneVarTerm, OneVarTermConstruction, TwoVarTerm, TwoVarTermConstruction},
        Vtype,
    },
    types::Bias,
};

use crate::common::*;

fn quadratic_expression_base(vtype: Vtype, n: usize) {
    let seed = make_seed();
    let env = create_env();
    let biases = random_biases::<Bias>(n, seed);
    let mut expr = create_linear_expression(env.clone(), &biases, vtype);

    let multiplier = env.add_variable("m", Some(vtype), None).unwrap();
    let mscalar = random_bias::<Bias>(seed);
    expr.mul_assign(&multiplier.mul(mscalar)).unwrap();

    let quadratic: Vec<TwoVarTerm> = biases
        .iter()
        .enumerate()
        // .map(|b| vec![OneVarTerm::new(multiplier.id, b * mscalar)])
        .map(|(i, b)| TwoVarTerm::new(i.into(), vec![OneVarTerm::new(multiplier.id, b * mscalar)]))
        .collect();
    // quadratic.push(vec![]);

    assert_eq!(expr.env, env, "envs is wrong");
    assert_eq!(expr.offset, Bias::default(), "offset is wrong");
    assert_eq!(
        expr.linear.to_vec(expr.num_variables),
        vec![Bias::default(); biases.len() + 1],
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
    // assert_eq!(
    //     expr.active.len(),
    //     biases.len() + 1,
    //     "the number of active variables in the result is false"
    // );
    // assert_eq!(
    //     expr.active,
    //     vec![true; biases.len() + 1],
    //     "all variables should be active in the result"
    // );
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
