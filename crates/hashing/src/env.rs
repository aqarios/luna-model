//! Deterministic hashing for environments.
//!
//! Variables are hashed in a canonical order (sorted by variable id) so the
//! result is deterministic regardless of the environment's internal
//! storage/iteration order.

use std::hash::Hasher;

use lunamodel_core::ArcEnv;
use lunamodel_types::{Bound, Vtype};

use crate::util::write_bytes;

fn write_bound(h: &mut impl Hasher, b: Bound) {
    match b {
        Bound::Bounded(v) => {
            h.write_u8(1);
            h.write_u64(v.to_bits());
        }
        Bound::Unbounded => h.write_u8(0),
    }
}

/// Hashes an environment's semantic content into `h`.
pub fn hash_env(env: &ArcEnv, h: &mut impl Hasher) {
    h.write(b"env");

    let mut vars = env.vars();
    vars.sort_unstable_by_key(|v| v.id());

    h.write_u64(vars.len() as u64);
    for var in vars {
        h.write_u32(var.id());
        write_bytes(h, var.name().unwrap().as_bytes());

        let vtype = var.vtype().unwrap();
        let vtype_tag: u8 = match vtype {
            Vtype::InvertedBinary => 0,
            Vtype::Binary => 1,
            Vtype::Spin => 2,
            Vtype::Integer => 3,
            Vtype::Real => 4,
        };
        h.write_u8(vtype_tag);

        if matches!(vtype, Vtype::Integer | Vtype::Real) {
            let bounds = var.bounds().unwrap();
            write_bound(h, bounds.lower);
            write_bound(h, bounds.upper);
        }
    }
}
