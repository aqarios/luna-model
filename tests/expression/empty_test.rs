use aqmodels::{
    core::{expression::ExpressionBaseCreation, Expression},
    types::Bias,
};

use crate::common::*;

#[test]
fn empty_expression() {
    let env = create_env();
    let expr = Expression::empty(env.clone());

    assert_eq!(expr.env, env);
    assert_eq!(expr.offset, Bias::default());
    assert_eq!(expr.linear.len(), 0);
    assert_eq!(expr.linear.to_vec(0), Vec::<Bias>::default());
    assert_eq!(expr.quadratic, None);
    assert_eq!(expr.higher_order, None);
    // assert_eq!(expr.active.len(), 0);
    // assert_eq!(expr.active, Vec::<bool>::default());
    assert_eq!(expr.num_variables, 0);
}
