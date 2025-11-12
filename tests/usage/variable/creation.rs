use crate::common::{assert_error, assert_noerror};
use luna_model::prelude::*;

#[test]
fn create_variable_explicit_direct() {
    let env = SharedEnvironment::default();
    _ = assert_noerror(env.add_binary("b"));
    _ = assert_noerror(env.add_spin("s"));
    _ = assert_noerror(env.add_integer("i", None));
    _ = assert_noerror(env.add_real("r", None));
}

#[test]
fn create_variable_explicit_lazy_bounds() {
    let env = SharedEnvironment::default();
    _ = assert_noerror(env.add_integer("i1", Some(LazyBounds::new(None, Some(Bound::Some(10.0))))));
    _ = assert_noerror(env.add_integer("i2", Some(LazyBounds::new(Some(Bound::Some(10.0)), None))));
    _ = assert_noerror(
        env.add_integer("i3", Some(LazyBounds::new(Some(Bound::Unbounded()), None))),
    );
    _ = assert_noerror(
        env.add_integer("i4", Some(LazyBounds::new(None, Some(Bound::Unbounded())))),
    );
    _ = assert_noerror(env.add_real("r1", Some(LazyBounds::new(None, Some(Bound::Some(10.0))))));
    _ = assert_noerror(env.add_real("r2", Some(LazyBounds::new(Some(Bound::Some(10.0)), None))));
    _ = assert_noerror(
        env.add_integer("r3", Some(LazyBounds::new(Some(Bound::Unbounded()), None))),
    );
    _ = assert_noerror(
        env.add_integer("r4", Some(LazyBounds::new(None, Some(Bound::Unbounded())))),
    );
}

#[test]
fn create_variable_explicit() {
    let env = SharedEnvironment::default();
    _ = assert_noerror(env.add_variable("b", Vtype::Binary, None));
    _ = assert_noerror(env.add_variable("s", Vtype::Spin, None));
    _ = assert_noerror(env.add_variable("i", Vtype::Integer, None));
    _ = assert_noerror(env.add_variable("r", Vtype::Real, None));
}

#[test]
fn create_variable_twice_with_delete() {
    let mut env = SharedEnvironment::default();

    let b = assert_noerror(env.add_variable("b", Vtype::Binary, None));
    let s = assert_noerror(env.add_variable("s", Vtype::Spin, None));
    let i = assert_noerror(env.add_variable("i", Vtype::Integer, None));
    let r = assert_noerror(env.add_variable("r", Vtype::Real, None));

    _ = env.remove(&b);
    _ = env.remove(&s);
    _ = env.remove(&i);
    _ = env.remove(&r);

    _ = assert_noerror(env.add_variable("b", Vtype::Binary, None));
    _ = assert_noerror(env.add_variable("s", Vtype::Spin, None));
    _ = assert_noerror(env.add_variable("i", Vtype::Integer, None));
    _ = assert_noerror(env.add_variable("r", Vtype::Real, None));
}

#[test]
fn create_variable_twice_diff_envs() {
    let env_a = SharedEnvironment::default();
    let env_b = SharedEnvironment::default();

    _ = assert_noerror(env_a.add_variable("b", Vtype::Binary, None));
    _ = assert_noerror(env_a.add_variable("s", Vtype::Spin, None));
    _ = assert_noerror(env_a.add_variable("i", Vtype::Integer, None));
    _ = assert_noerror(env_a.add_variable("r", Vtype::Real, None));

    _ = assert_noerror(env_b.add_variable("b", Vtype::Binary, None));
    _ = assert_noerror(env_b.add_variable("s", Vtype::Spin, None));
    _ = assert_noerror(env_b.add_variable("i", Vtype::Integer, None));
    _ = assert_noerror(env_b.add_variable("r", Vtype::Real, None));
}

#[test]
fn create_variable_invalid_name() {
    let env = SharedEnvironment::default();
    _ = assert_error(
        env.add_binary("0b"),
        errors::VariableCreationErr::VarName(
            "Variable names must start with an alphabetic character.".to_owned(),
        ),
    );
    _ = assert_error(
        env.add_binary("b-1233"),
        errors::VariableCreationErr::VarName(
            "Variable names must only contain alphanumeric characters or '_' or ','.".to_owned(),
        ),
    );
    _ = assert_error(
        env.add_binary("xß"),
        errors::VariableCreationErr::VarName(
            "Variable names must only contain alphanumeric characters or '_' or ','.".to_owned(),
        ),
    );
}
