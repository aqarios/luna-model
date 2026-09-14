//! Deterministic hashing for constraint collections.
//!
//! Constraints are explicitly sorted by name before hashing so the result
//! does not depend on `ConstraintCollection`'s insertion-order-based
//! iteration - two models with the same constraints added in a different
//! order hash identically.

use std::hash::Hasher;

use lunamodel_core::{Constraint, ConstraintCollection};
use lunamodel_types::Comparator;

use crate::expr::hash_expr;
use crate::util::write_bytes;

/// Hashes a constraint collection's semantic content into `h`.
pub fn hash_constr(constr: &ConstraintCollection, h: &mut impl Hasher) {
    h.write(b"constr");

    let mut entries: Vec<(&String, &Constraint)> = constr.iter().collect();
    entries.sort_unstable_by_key(|(name, _)| *name);

    h.write_u64(entries.len() as u64);
    for (name, c) in entries {
        write_bytes(h, name.as_bytes());
        hash_expr(&c.lhs, h);
        h.write_u64(c.rhs.to_bits());
        let cmp: u32 = match c.comparator {
            Comparator::Le => 0,
            Comparator::Eq => 1,
            Comparator::Ge => 2,
        };
        h.write_u32(cmp);
    }
}
