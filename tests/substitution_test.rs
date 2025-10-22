mod common;

use luna_model::core::{Bound, LazyBounds, Model, Substitution};
use luna_model::errors::VariableNotExistingErr;
use common::create_env;

#[test]
fn substitute_integer_single() {
    let env = create_env();

    let a = &env
        .add_integer("a", None)
        .expect("adding 'a' to env failed.");
    let target = &env
        .add_integer("target", None)
        .expect("adding 'b' to env failed.");
    let base = (a * 3.4 + 10.10 * target).expect("creating base expression failed.");

    let b1 = &env.add_binary("b1").expect("adding 'b1' to env failed.");
    let b2 = &env.add_binary("b2").expect("adding 'b2' to env failed.");
    let b3 = &env.add_binary("b3").expect("adding 'b3' to env failed.");
    let replacement = (b1 + b2 + b3).expect("Sum b1, b2 and b3 failed.");

    let expected =
        (3.4 * a + 10.10 * (b1 + b2 + b3)).expect("creating expected expression failed.");

    let result = (&base)
        .substitute(target, &replacement)
        .expect("substitute failed.");
    assert_eq!(expected, result);
}

#[test]
fn substitute_integer_squared() {
    let env = create_env();

    let target = &env
        .add_integer(
            "n",
            Some(LazyBounds::new(
                Some(Bound::Some(0.0)),
                Some(Bound::Some(7.0)),
            )),
        )
        .expect("adding 'a' to env failed.");
    let base = (target * target).expect("creating base expression failed.");

    let x1 = &env.add_binary("x_1").expect("adding 'b1' to env failed.");
    let x2 = &env.add_binary("x_2").expect("adding 'b2' to env failed.");
    let x3 = &env.add_binary("x_3").expect("adding 'b3' to env failed.");
    let replacement = (x1 + 2 * x2 + 4 * x3).expect("Sum b1, b2 and b3 failed.");

    let expected = (x1 + 4 * x2 + 16 * x3 + 4 * x1 * x2 + 8 * x1 * x3 + 16 * x2 * x3)
        .expect("creating expected expression failed.");

    let result = (&base)
        .substitute(target, &replacement)
        .expect("substitute failed.");
    println!("base = {}", base);
    println!("target = {}", target);
    println!("replacement = {}", replacement);
    println!("-----");
    println!("expected = {}", expected);
    println!("result = {}", result);
    assert_eq!(expected, result);
}

#[test]
fn substitute_integer_higher_order() {
    let env = create_env();

    let target = &env
        .add_integer(
            "n",
            Some(LazyBounds::new(
                Some(Bound::Some(0.0)),
                Some(Bound::Some(7.0)),
            )),
        )
        .expect("adding 'a' to env failed.");
    let base = (target * target * target).expect("creating base expression failed.");

    let x1 = &env.add_binary("x_1").expect("adding 'b1' to env failed.");
    let x2 = &env.add_binary("x_2").expect("adding 'b2' to env failed.");
    let x3 = &env.add_binary("x_3").expect("adding 'b3' to env failed.");
    let replacement = (x1 + 2 * x2 + 4 * x3).expect("Sum b1, b2 and b3 failed.");

    let expected = ((x1 ^ 3)
        + 6 * (x1 ^ 2) * x2
        + 12 * (x1 ^ 2) * x3
        + 12 * (x2 ^ 2) * x1
        + 48 * x1 * x2 * x3
        + 48 * (x3 ^ 2) * x1
        + 8 * (x2 ^ 3)
        + 48 * (x2 ^ 2) * x3
        + 96 * (x3 ^ 2) * x2
        + 64 * (x3 ^ 3))
        .expect("creating expected expression failed.");

    let result = (&base)
        .substitute(target, &replacement)
        .expect("substitute failed.");
    println!("base = {}", base);
    println!("target = {}", target);
    println!("replacement = {}", replacement);
    println!("-----");
    println!("expected = {}", expected);
    println!("result   = {}", result);
    assert_eq!(expected, result);
}

// MODEL

#[test]
fn substitute_integer_single_model() {
    let mut model = Model::default();
    let a = &model
        .environment
        .add_integer("a", None)
        .expect("adding 'a' to env failed.");
    let target = &model
        .environment
        .add_integer("target", None)
        .expect("adding 'b' to env failed.");

    model.objective = (a * 3.4 + 10.10 * target).expect("creating base expression failed.");

    let b1 = &model
        .environment
        .add_binary("b1")
        .expect("adding 'b1' to env failed.");
    let b2 = &model
        .environment
        .add_binary("b2")
        .expect("adding 'b2' to env failed.");
    let b3 = &model
        .environment
        .add_binary("b3")
        .expect("adding 'b3' to env failed.");
    let replacement = (b1 + b2 + b3).expect("Sum b1, b2 and b3 failed.");

    let expected =
        (3.4 * a + 10.10 * (b1 + b2 + b3)).expect("creating expected expression failed.");

    model
        .substitute(target, &replacement)
        .expect("substitute failed.");
    assert_eq!(expected, model.objective);
    let result = model.environment.get_vref_by_name("target");
    match result {
        Ok(_) => panic!("target should not be contained in model after substitution."),
        Err(err) => assert_eq!(err, VariableNotExistingErr {}),
    };
}

#[test]
fn substitute_integer_squared_model() {
    let mut model = Model::default();

    let target = &model
        .environment
        .add_integer(
            "target",
            Some(LazyBounds::new(
                Some(Bound::Some(0.0)),
                Some(Bound::Some(7.0)),
            )),
        )
        .expect("adding 'a' to env failed.");
    model.objective = (target * target).expect("creating base expression failed.");

    let x1 = &model
        .environment
        .add_binary("x_1")
        .expect("adding 'b1' to env failed.");
    let x2 = &model
        .environment
        .add_binary("x_2")
        .expect("adding 'b2' to env failed.");
    let x3 = &model
        .environment
        .add_binary("x_3")
        .expect("adding 'b3' to env failed.");
    let replacement = (x1 + 2 * x2 + 4 * x3).expect("Sum b1, b2 and b3 failed.");

    let expected = (x1 + 4 * x2 + 16 * x3 + 4 * x1 * x2 + 8 * x1 * x3 + 16 * x2 * x3)
        .expect("creating expected expression failed.");

    model
        .substitute(target, &replacement)
        .expect("substitute failed.");
    assert_eq!(expected, model.objective);
    let result = model.environment.get_vref_by_name("target");
    match result {
        Ok(_) => panic!("target should not be contained in model after substitution."),
        Err(err) => assert_eq!(err, VariableNotExistingErr {}),
    };
}

#[test]
fn substitute_integer_higher_order_model() {
    let mut model = Model::default();

    let target = &model
        .environment
        .add_integer(
            "target",
            Some(LazyBounds::new(
                Some(Bound::Some(0.0)),
                Some(Bound::Some(7.0)),
            )),
        )
        .expect("adding 'a' to env failed.");
    model.objective = (target * target * target).expect("creating base expression failed.");

    let x1 = &model
        .environment
        .add_binary("x_1")
        .expect("adding 'b1' to env failed.");
    let x2 = &model
        .environment
        .add_binary("x_2")
        .expect("adding 'b2' to env failed.");
    let x3 = &model
        .environment
        .add_binary("x_3")
        .expect("adding 'b3' to env failed.");
    let replacement = (x1 + 2 * x2 + 4 * x3).expect("Sum b1, b2 and b3 failed.");

    let expected = ((x1 ^ 3)
        + 6 * (x1 ^ 2) * x2
        + 12 * (x1 ^ 2) * x3
        + 12 * (x2 ^ 2) * x1
        + 48 * x1 * x2 * x3
        + 48 * (x3 ^ 2) * x1
        + 8 * (x2 ^ 3)
        + 48 * (x2 ^ 2) * x3
        + 96 * (x3 ^ 2) * x2
        + 64 * (x3 ^ 3))
        .expect("creating expected expression failed.");

    model
        .substitute(target, &replacement)
        .expect("substitute failed.");
    assert_eq!(expected, model.objective);
    let result = model.environment.get_vref_by_name("target");
    match result {
        Ok(_) => panic!("target should not be contained in model after substitution."),
        Err(err) => assert_eq!(err, VariableNotExistingErr {}),
    };
}
