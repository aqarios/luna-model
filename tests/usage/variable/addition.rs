use luna_model::prelude::*;

use crate::common::assert_noerror;

#[test]
fn with_number() {
    let mut env = SharedEnvironment::default();

    fn for_vtype(vtype: Vtype, env: &mut SharedEnvironment) {
        let x = assert_noerror(env.add_variable("x", vtype, None));
        let res = 10198342.0 + &x;
        assert_eq!(1, res.num_variables);

        let linear_factor = assert_noerror(res.linear(x.id));
        assert_eq!(1.0, linear_factor);
        assert_eq!(10198342.0, res.offset);
        assert_eq!(10198342.0, res.offset());

        env.remove(&x);
    }

    for_vtype(Vtype::Binary, &mut env);
    for_vtype(Vtype::InvertedBinary, &mut env);
    for_vtype(Vtype::Spin, &mut env);
    for_vtype(Vtype::Integer, &mut env);
    for_vtype(Vtype::Real, &mut env);
}

#[test]
fn with_number_r() {
    let mut env = SharedEnvironment::default();

    fn for_vtype(vtype: Vtype, env: &mut SharedEnvironment) {
        let x = assert_noerror(env.add_variable("x", vtype, None));
        let res = &x + 10198342.0;
        assert_eq!(1, res.num_variables);

        let linear_factor = assert_noerror(res.linear(x.id));
        assert_eq!(1.0, linear_factor);
        assert_eq!(10198342.0, res.offset);
        assert_eq!(10198342.0, res.offset());

        env.remove(&x);
    }

    for_vtype(Vtype::Binary, &mut env);
    for_vtype(Vtype::InvertedBinary, &mut env);
    for_vtype(Vtype::Spin, &mut env);
    for_vtype(Vtype::Integer, &mut env);
    for_vtype(Vtype::Real, &mut env);
}

#[test]
fn with_other() {
    let mut env = SharedEnvironment::default();

    fn for_vtype(vtype: Vtype, env: &mut SharedEnvironment) {
        let x = assert_noerror(env.add_variable("x", vtype, None));
        let y = assert_noerror(env.add_variable("y", vtype, None));
        let res = assert_noerror((&x + &y).into_result());
        assert_eq!(2, res.num_variables);

        let linear_factor_x = assert_noerror(res.linear(x.id));
        let linear_factor_y = assert_noerror(res.linear(y.id));
        assert_eq!(1.0, linear_factor_x);
        assert_eq!(1.0, linear_factor_y);
        assert_eq!(0.0, res.offset);
        assert_eq!(0.0, res.offset());

        env.remove(&x);
        env.remove(&y);
    }

    for_vtype(Vtype::Binary, &mut env);
    for_vtype(Vtype::InvertedBinary, &mut env);
    for_vtype(Vtype::Spin, &mut env);
    for_vtype(Vtype::Integer, &mut env);
    for_vtype(Vtype::Real, &mut env);
}

#[test]
fn with_other_r() {
    let mut env = SharedEnvironment::default();

    fn for_vtype(vtype: Vtype, env: &mut SharedEnvironment) {
        let x = assert_noerror(env.add_variable("x", vtype, None));
        let y = assert_noerror(env.add_variable("y", vtype, None));
        let res = assert_noerror((&y + &x).into_result());
        assert_eq!(2, res.num_variables);

        let linear_factor_x = assert_noerror(res.linear(x.id));
        let linear_factor_y = assert_noerror(res.linear(y.id));
        assert_eq!(1.0, linear_factor_x);
        assert_eq!(1.0, linear_factor_y);
        assert_eq!(0.0, res.offset);
        assert_eq!(0.0, res.offset());

        env.remove(&x);
        env.remove(&y);
    }

    for_vtype(Vtype::Binary, &mut env);
    for_vtype(Vtype::InvertedBinary, &mut env);
    for_vtype(Vtype::Spin, &mut env);
    for_vtype(Vtype::Integer, &mut env);
    for_vtype(Vtype::Real, &mut env);
}

#[test]
fn with_other_sel_last() {
    let mut env = SharedEnvironment::default();

    fn for_vtype(vtype: Vtype, env: &mut SharedEnvironment) {
        let r1 = assert_noerror(env.add_variable("r1", vtype, None));
        let r2 = assert_noerror(env.add_variable("r2", vtype, None));
        let x = assert_noerror(env.add_variable("x", vtype, None));
        let y = assert_noerror(env.add_variable("y", vtype, None));
        _ = assert_noerror((&x + &y).into_result());
        let res = assert_noerror((&y + &x).into_result());
        assert_eq!(2, res.num_variables);

        let linear_factor_x = assert_noerror(res.linear(x.id));
        let linear_factor_y = assert_noerror(res.linear(y.id));
        assert_eq!(1.0, linear_factor_x);
        assert_eq!(1.0, linear_factor_y);
        assert_eq!(0.0, res.offset);
        assert_eq!(0.0, res.offset());

        env.remove(&r1);
        env.remove(&r2);
        env.remove(&x);
        env.remove(&y);
    }

    for_vtype(Vtype::Binary, &mut env);
    for_vtype(Vtype::InvertedBinary, &mut env);
    for_vtype(Vtype::Spin, &mut env);
    for_vtype(Vtype::Integer, &mut env);
    for_vtype(Vtype::Real, &mut env);
}

#[test]
fn with_other_sel() {
    let mut env = SharedEnvironment::default();

    fn for_vtype(vtype: Vtype, env: &mut SharedEnvironment) {
        let r1 = assert_noerror(env.add_variable("r1", vtype, None));
        let x = assert_noerror(env.add_variable("x", vtype, None));
        let r2 = assert_noerror(env.add_variable("r2", vtype, None));
        let y = assert_noerror(env.add_variable("y", vtype, None));
        _ = assert_noerror((&x + &y).into_result());
        let res = assert_noerror((&y + &x).into_result());
        assert_eq!(2, res.num_variables);

        let linear_factor_x = assert_noerror(res.linear(x.id));
        let linear_factor_y = assert_noerror(res.linear(y.id));
        assert_eq!(1.0, linear_factor_x);
        assert_eq!(1.0, linear_factor_y);
        assert_eq!(0.0, res.offset);
        assert_eq!(0.0, res.offset());

        env.remove(&r1);
        env.remove(&r2);
        env.remove(&x);
        env.remove(&y);
    }

    for_vtype(Vtype::Binary, &mut env);
    for_vtype(Vtype::InvertedBinary, &mut env);
    for_vtype(Vtype::Spin, &mut env);
    for_vtype(Vtype::Integer, &mut env);
    for_vtype(Vtype::Real, &mut env);
}
