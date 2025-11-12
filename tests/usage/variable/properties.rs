use luna_model::prelude::*;

use crate::common::assert_noerror;

#[test]
fn properties() {
    let env = SharedEnvironment::default();
    let lower = Bound::Some(-5.0);
    let upper = Bound::Some(42.0);
    let x = assert_noerror(env.add_variable(
        "x",
        Vtype::Integer,
        Some(LazyBounds::new(Some(lower), Some(upper))),
    ));
    let var = &env.access()[x.id];
    assert_eq!(Vtype::Integer, var.vtype);
    assert_eq!("x", var.name);
    assert_eq!(lower, var.bounds.lower);
    assert_eq!(upper, var.bounds.upper);
}
