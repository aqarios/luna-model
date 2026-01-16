use std::hash::{DefaultHasher, Hash, Hasher};

use lunamodel_core::Model;

mod constr;
mod env;
mod expr;
mod model;

use model::HashModel;

pub fn hash_model(model: &Model) -> u64 {
    let mut s = DefaultHasher::new();
    let hashmodel = HashModel::build(&model);
    hashmodel.hash(&mut s);
    s.finish()
}
