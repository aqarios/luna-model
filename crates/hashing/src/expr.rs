//! Deterministic hashing for expressions.
//!
//! Only an expression's actual nonzero terms are hashed - never a buffer
//! sized to the model's variable-ID space - so cost is proportional to the
//! number of nonzero terms, not to how large a variable id happens to be.
//! Every term list is explicitly sorted by a stable key before hashing, so
//! the result does not depend on the internal storage's iteration order
//! (linear/quadratic storage already happens to iterate in sorted order
//! today, but higher-order storage is a `HashMap` with no ordering
//! guarantee at all - sorting defensively here means this code keeps working
//! if any of those internal representations change).

use std::hash::Hasher;

use lunamodel_core::Expression;

/// Hashes an expression's semantic content into `h`.
pub fn hash_expr(expr: &Expression, h: &mut impl Hasher) {
    h.write(b"expr");
    h.write_u64(expr.offset.to_bits());

    let mut linear: Vec<(u32, f64)> = expr.raw_linear_items().collect();
    linear.sort_unstable_by_key(|(id, _)| *id);
    h.write_u64(linear.len() as u64);
    for (id, bias) in linear {
        h.write_u32(id);
        h.write_u64(bias.to_bits());
    }

    let mut quad: Vec<(u32, u32, f64)> = expr
        .raw_quadratic_items()
        .map(|(u, v, bias)| if u <= v { (u, v, bias) } else { (v, u, bias) })
        .collect();
    quad.sort_unstable_by_key(|(lo, hi, _)| (*lo, *hi));
    h.write_u64(quad.len() as u64);
    for (lo, hi, bias) in quad {
        h.write_u32(lo);
        h.write_u32(hi);
        h.write_u64(bias.to_bits());
    }

    let mut higher_order: Vec<(Vec<u32>, f64)> = expr
        .raw_higher_order_items()
        .map(|(mut ids, bias)| {
            ids.sort_unstable();
            (ids, bias)
        })
        .collect();
    higher_order.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));
    h.write_u64(higher_order.len() as u64);
    for (ids, bias) in higher_order {
        h.write_u64(ids.len() as u64);
        for id in ids {
            h.write_u32(id);
        }
        h.write_u64(bias.to_bits());
    }
}
