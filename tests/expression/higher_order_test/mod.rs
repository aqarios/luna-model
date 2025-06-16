mod binary_test;
mod integer_test;
mod real_test;
mod spin_test;

use hashbrown::HashMap;

use aqmodels::{core::{
    operations::{MulAssignToExpression, MulToExpression},
    term::{types::OneVarTerm, HigherOrder},
    Vtype,
}, types::Bias};

use crate::common::*;

fn higher_order_expression_base(vtype: Vtype, n: usize) {
    let seed = make_seed();
    let env = create_env();
    let biases = random_biases::<Bias>(n, seed);
    let (mut expr, vars) = create_linear_expression_with_vars(env.clone(), &biases, vtype);

    let ma = env.add_variable("ma", Some(vtype), None).unwrap();
    let mb = env.add_variable("mb", Some(vtype), None).unwrap();
    let ma_scalar = random_bias::<Bias>(seed);
    let mb_scalar = random_bias::<Bias>(seed);

    expr.mul_assign(&ma.mul(ma_scalar)).unwrap();
    println!("expr.linear after first mul {:?}", &expr.linear.to_vec());
    println!(
        "expr.adj after first mul {:?}",
        &expr.quadratic.as_ref().unwrap().adj
    );
    expr.mul_assign(&mb.mul(mb_scalar)).unwrap();
    println!("expr.linear after second mul {:?}", &expr.linear.to_vec());
    println!(
        "expr.adj after second mul {:?}",
        &expr.quadratic.as_ref().unwrap().adj
    );
    println!(
        "expr.ho after second mul {:?}",
        &expr.higher_order.as_ref().unwrap().biases
    );

    let expected_quadratic: Vec<Vec<OneVarTerm>> = vec![vec![]; biases.len() + 2];

    let mut expected_higher_order: HashMap<String, Bias> = HashMap::with_capacity(biases.len());
    for (var, bias) in vars.iter().zip(&biases) {
        let key = HigherOrder::make_key(&vec![var.id, ma.id, mb.id]);
        expected_higher_order.insert(key, *bias * ma_scalar * mb_scalar);
    }

    assert_eq!(expr.env, env, "envs is wrong");
    assert_eq!(expr.offset, Bias::default(), "offset is wrong");
    assert_eq!(
        expr.linear.to_vec(),
        &vec![Bias::default(); biases.len() + 2],
        "linear parts are not equal"
    );
    if expr.has_quadratic() {
        assert_eq!(expr.quadratic.as_ref().unwrap().adj, expected_quadratic,);
    }
    assert_ne!(expr.higher_order, None, "higher order should be None");
    assert_eq!(
        expr.higher_order.unwrap().biases,
        expected_higher_order,
        "higher order not as expected"
    );
    assert_eq!(
        expr.active.len(),
        biases.len() + 2,
        "the number of active variables in the result is false"
    );
    assert_eq!(
        expr.active,
        vec![true; biases.len() + 2],
        "all variables should be active in the result"
    );
    assert_eq!(
        expr.num_variables,
        biases.len() + 2,
        "the number of variables should have increased by one"
    );
}

#[test]
fn higher_order_expression() {
    let n = 100;
    higher_order_expression_base(Vtype::Binary, n);
    higher_order_expression_base(Vtype::Spin, n);
    higher_order_expression_base(Vtype::Integer, n);
    higher_order_expression_base(Vtype::Real, n);
}
