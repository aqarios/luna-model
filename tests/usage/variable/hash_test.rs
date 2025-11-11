use std::hash::{DefaultHasher, Hash, Hasher};

use crate::common::assert_noerror;
use luna_model::prelude::*;


#[test]
fn compute_variable_hash() {
    let env = SharedEnvironment::default();
    let b = assert_noerror(env.add_binary("x"));
    let var = &env.access()[b.id];
    let mut s = DefaultHasher::new();
    var.name.hash(&mut s);
    let hash_value = s.finish();
    assert_eq!(8312289520117458465, hash_value)
}
