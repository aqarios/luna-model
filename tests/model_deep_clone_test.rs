mod common;

use aqmodels::core::{operations::{AddAssignToExpression, MulAssignToExpression}, Model};
use common::*;

#[test]
fn model_deep_clone() {
    let mut m1 = Model::default();
    let x = assert_noerror(m1.environment.add_binary("x"));
    let y = assert_noerror(m1.environment.add_binary("y"));
    assert_noerror(m1.objective.add_assign(&x));
    assert_noerror(m1.objective.mul_assign(&y));

    let m2 = m1.deep_clone();
    assert_ne!(m1, m2)
}
