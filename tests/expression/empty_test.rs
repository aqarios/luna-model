use std::rc::Rc;

use aqmodels::core::{expression::ExpressionBaseCreation, Expression, VarId};

use crate::common::*;

#[test]
fn empty_expression() {
    let env = package(create_env());
    let expr = Expression::<VarId, f64>::empty(Rc::clone(&env));

    assert_eq!(expr.env, env);
    assert_eq!(expr.offset, f64::default());
    assert_eq!(expr.linear.len(), 0);
    assert_eq!(expr.linear.to_vec(), &Vec::<f64>::default());
    assert_eq!(expr.quadratic, None);
    assert_eq!(expr.higher_order, None);
    assert_eq!(expr.active.len(), 0);
    assert_eq!(expr.active, Vec::<bool>::default());
    assert_eq!(expr.num_variables, 0);
}
