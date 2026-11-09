//! Decoded data comes from untrusted bytes, so lengths, indices and counts are
//! checked before use: malformed data is a decoding error, never a panic.

use std::{
    collections::HashMap,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use lunamodel_core::{
    Solution,
    prelude::{LazyBounds, Variable},
};
use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::{VarIdx, Vtype};

pub fn corrupt(what: &str) -> LunaModelError {
    LunaModelError::Decoding(format!("corrupt data: {what}").into())
}

/// `values[i]`, or an error naming what is missing.
pub fn at<T: Clone>(values: &[T], i: usize, what: &str) -> LunaModelResult<T> {
    values.get(i).cloned().ok_or_else(|| corrupt(what))
}

/// `values[start..start + len]`, or an error naming what is missing.
pub fn span<'a, T>(
    values: &'a [T],
    start: usize,
    len: usize,
    what: &str,
) -> LunaModelResult<&'a [T]> {
    start
        .checked_add(len)
        .and_then(|end| values.get(start..end))
        .ok_or_else(|| corrupt(what))
}

/// The next value of a list that is consumed in order.
pub fn next<T>(values: &mut impl Iterator<Item = T>, what: &str) -> LunaModelResult<T> {
    values.next().ok_or_else(|| corrupt(what))
}

/// `idx`, if the environment ever handed it out; `next_idx` is the first it has not.
/// Removed variables still count, as expressions may refer to them.
pub fn var(idx: VarIdx, next_idx: VarIdx) -> LunaModelResult<VarIdx> {
    match idx < next_idx {
        true => Ok(idx),
        false => Err(corrupt("unknown variable")),
    }
}

/// The first variable index after all of `indices`.
pub fn first_unused<'a>(indices: impl Iterator<Item = &'a VarIdx>) -> LunaModelResult<VarIdx> {
    match indices.max() {
        Some(max) => max
            .checked_add(1)
            .ok_or_else(|| corrupt("variable index out of range")),
        None => Ok(0),
    }
}

/// `a * b`, unless it overflows.
pub fn product(a: u64, b: u64, what: &str) -> LunaModelResult<usize> {
    a.checked_mul(b)
        .and_then(|n| usize::try_from(n).ok())
        .ok_or_else(|| corrupt(what))
}

/// A decoded variable; an invalid name is corrupt data.
pub fn variable(name: &str, vtype: Vtype, bounds: Option<LazyBounds>) -> LunaModelResult<Variable> {
    Variable::new(name, vtype, bounds).map_err(|e| corrupt(&e.to_string()))
}

/// Adds a decoded variable; a repeated index or name is corrupt data.
pub fn insert(
    variables: &mut HashMap<VarIdx, Variable>,
    lookup: &mut HashMap<String, VarIdx>,
    idx: VarIdx,
    var: Variable,
    name: String,
) -> LunaModelResult<()> {
    match variables.insert(idx, var).is_none() && lookup.insert(name, idx).is_none() {
        true => Ok(()),
        false => Err(corrupt("repeated variable")),
    }
}

/// The time `secs` seconds after the Unix epoch.
pub fn time(secs: f64) -> LunaModelResult<SystemTime> {
    Duration::try_from_secs_f64(secs)
        .ok()
        .and_then(|duration| UNIX_EPOCH.checked_add(duration))
        .ok_or_else(|| corrupt("invalid time"))
}

/// Every constraint and bound result has one value per sample, as the encoder expects.
pub fn aligned(sol: &Solution) -> LunaModelResult<()> {
    let mut results = sol.constraints.values().chain(sol.variable_bounds.values());
    match results.all(|values| values.len() == sol.len()) {
        true => Ok(()),
        false => Err(corrupt("results do not match the samples")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn times_outside_the_representable_range_are_an_error() {
        assert!(time(1.5).is_ok());
        for secs in [-1.0, f64::NAN, f64::INFINITY, 1e300] {
            assert!(time(secs).is_err());
        }
    }
}
