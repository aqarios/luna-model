//! Deterministic hashing for models.

use std::hash::Hasher;

use lunamodel_core::Model;
use lunamodel_types::Sense;

use crate::constr::hash_constr;
use crate::env::hash_env;
use crate::expr::hash_expr;
use crate::util::write_bytes;

/// Hashes a model's semantic content into `h`.
pub fn hash_model(model: &Model, h: &mut impl Hasher) {
    h.write(b"model");

    write_bytes(h, model.name.as_bytes());

    let sense_tag: u8 = match model.sense {
        Sense::Min => 0,
        Sense::Max => 1,
    };
    h.write_u8(sense_tag);

    hash_expr(&model.objective, h);
    hash_constr(&model.constraints, h);
    hash_env(&model.environment, h);
}
