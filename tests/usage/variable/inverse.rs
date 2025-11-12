use crate::common::{assert_error, assert_noerror};
use itertools::Itertools;
use luna_model::prelude::*;
use std::ops::Not;

// //////////////////////////////////// //
// //// Invert with operator tests //// //
// //////////////////////////////////// //

#[test]
fn inverse_binary() {
    let env = SharedEnvironment::default();
    let b = assert_noerror(env.add_binary("b"));
    let b_inv = assert_noerror(!&b);
    let b_inv_fn = assert_noerror((&b).not());
    assert_eq!(&b_inv, &b_inv_fn);

    let locked = env.access();
    let var_b = &locked[b.id];
    let var_b_inv = &locked[b_inv.id];
    let var_b_inv_fn = &locked[b_inv_fn.id];

    assert_eq!(Some(b.id), var_b_inv.inverted);
    assert_eq!(Some(b.id), var_b_inv_fn.inverted);
    assert_eq!(Some(b_inv.id), var_b.inverted);
}

#[test]
fn inverse_binary_linear() {
    let env = SharedEnvironment::default();
    let b = assert_noerror(env.add_binary("b"));
    let expr = 5 * &(!&b).unwrap();
    let mut sol = Solution::default();
    sol.create_columns(&env, 1);
    assert_noerror(sol.extend(&Vec::from([0]), 1, 0.0));
    let sample = &sol.iter_samples().map(|s| s).collect_vec()[0];
    let value = expr.evaluate_sample(sample, |i| i);
    assert_eq!(5.0, value)
}

#[test]
fn inverse_binary_self_quad() {
    let env = SharedEnvironment::default();
    let b = assert_noerror(env.add_binary("b"));
    let expr = assert_noerror((5 * &b * &(!&b).unwrap()).into_result());
    let mut sol = Solution::default();
    sol.create_columns(&env, 1);
    assert_noerror(sol.extend(&Vec::from([0]), 1, 0.0));
    let sample = &sol.iter_samples().map(|s| s).collect_vec()[0];
    let value = expr.evaluate_sample(sample, |i| i);
    assert_eq!(0.0, value)
}

#[test]
fn inverse_binary_self_quad_reversed() {
    let env = SharedEnvironment::default();
    let b = assert_noerror(env.add_binary("b"));
    let expr = assert_noerror((5 * &(!&b).unwrap() * &b).into_result());
    let mut sol = Solution::default();
    sol.create_columns(&env, 1);
    assert_noerror(sol.extend(&Vec::from([0]), 1, 0.0));
    let sample = &sol.iter_samples().map(|s| s).collect_vec()[0];
    let value = expr.evaluate_sample(sample, |i| i);
    assert_eq!(0.0, value)
}

#[test]
fn inverse_binary_higher_order() {
    let env = SharedEnvironment::default();
    let a = assert_noerror(env.add_binary("a"));
    let b = assert_noerror(env.add_binary("b"));
    let c = assert_noerror(env.add_binary("c"));

    fn eval_and_check(e: &Expression, ev: &SharedEnvironment) {
        let mut sol = Solution::default();
        sol.create_columns(ev, 1);
        assert_noerror(sol.extend(&Vec::from([1, 0, 1]), 1, 0.0));
        let sample = &sol.iter_samples().map(|s| s).collect_vec()[0];
        let value = e.evaluate_sample(sample, |i| i);
        assert_eq!(0.0, value)
    }

    eval_and_check(
        &assert_noerror((5 * &a * &b * &c * &(!&b).unwrap()).into_result()),
        &env,
    );
    eval_and_check(
        &assert_noerror((5 * &a * &c * &b * &(!&b).unwrap()).into_result()),
        &env,
    );
    eval_and_check(
        &assert_noerror((5 * &a * &(!&b).unwrap() * &c * &b).into_result()),
        &env,
    );
}

#[test]
fn inverse_unssupported() {
    let env = SharedEnvironment::default();
    let a = assert_noerror(env.add_spin("a"));
    let b = assert_noerror(env.add_integer("b", None));
    let c = assert_noerror(env.add_real("c", None));

    assert_error(!&a, errors::UnsupportedNotOperationErr::new(Vtype::Spin));
    assert_error(!&b, errors::UnsupportedNotOperationErr::new(Vtype::Integer));
    assert_error(!&c, errors::UnsupportedNotOperationErr::new(Vtype::Real));
}

// //////////////////////////////////// //
// ///// Invert with method tests ///// //
// //////////////////////////////////// //

#[test]
fn inverse_binary_linear_explicit() {
    let env = SharedEnvironment::default();
    let b = assert_noerror(env.add_binary("b"));
    let expr = 5 * &b.not().unwrap();
    let mut sol = Solution::default();
    sol.create_columns(&env, 1);
    assert_noerror(sol.extend(&Vec::from([0]), 1, 0.0));
    let sample = &sol.iter_samples().map(|s| s).collect_vec()[0];
    let value = expr.evaluate_sample(sample, |i| i);
    assert_eq!(5.0, value)
}

#[test]
fn inverse_binary_self_quad_explicit() {
    let env = SharedEnvironment::default();
    let b = assert_noerror(env.add_binary("b"));
    let expr = assert_noerror((5 * &b * &b.not().unwrap()).into_result());
    let mut sol = Solution::default();
    sol.create_columns(&env, 1);
    assert_noerror(sol.extend(&Vec::from([0]), 1, 0.0));
    let sample = &sol.iter_samples().map(|s| s).collect_vec()[0];
    let value = expr.evaluate_sample(sample, |i| i);
    assert_eq!(0.0, value)
}

#[test]
fn inverse_binary_self_quad_reversed_explicit() {
    let env = SharedEnvironment::default();
    let b = assert_noerror(env.add_binary("b"));
    let expr = assert_noerror((5 * &b.not().unwrap() * &b).into_result());
    let mut sol = Solution::default();
    sol.create_columns(&env, 1);
    assert_noerror(sol.extend(&Vec::from([0]), 1, 0.0));
    let sample = &sol.iter_samples().map(|s| s).collect_vec()[0];
    let value = expr.evaluate_sample(sample, |i| i);
    assert_eq!(0.0, value)
}

#[test]
fn inverse_binary_higher_order_explicit() {
    let env = SharedEnvironment::default();
    let a = assert_noerror(env.add_binary("a"));
    let b = assert_noerror(env.add_binary("b"));
    let c = assert_noerror(env.add_binary("c"));

    fn eval_and_check(e: &Expression, ev: &SharedEnvironment) {
        let mut sol = Solution::default();
        sol.create_columns(ev, 1);
        assert_noerror(sol.extend(&Vec::from([1, 0, 1]), 1, 0.0));
        let sample = &sol.iter_samples().map(|s| s).collect_vec()[0];
        let value = e.evaluate_sample(sample, |i| i);
        assert_eq!(0.0, value)
    }

    eval_and_check(
        &assert_noerror((5 * &a * &b * &c * &b.not().unwrap()).into_result()),
        &env,
    );
    eval_and_check(
        &assert_noerror((5 * &a * &c * &b * &b.not().unwrap()).into_result()),
        &env,
    );
    eval_and_check(
        &assert_noerror((5 * &a * &b.not().unwrap() * &c * &b).into_result()),
        &env,
    );
}

#[test]
fn inverse_unssupported_explicit() {
    let env = SharedEnvironment::default();
    let a = assert_noerror(env.add_spin("a"));
    let b = assert_noerror(env.add_integer("b", None));
    let c = assert_noerror(env.add_real("c", None));

    assert_error(a.not(), errors::UnsupportedNotOperationErr::new(Vtype::Spin));
    assert_error(b.not(), errors::UnsupportedNotOperationErr::new(Vtype::Integer));
    assert_error(c.not(), errors::UnsupportedNotOperationErr::new(Vtype::Real));
}
