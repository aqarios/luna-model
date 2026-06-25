//! Capsule ABI versioning for the Python FFI bridge.
//!
//! Rust has no stable ABI, so the raw-pointer [`pyo3::types::PyCapsule`]s
//! exchanged between the host `luna_model` library and separately-compiled
//! extensions (built with `pyo3-lunamodel`) are only sound when both sides
//! share the exact in-memory layout of every type that crosses the boundary.
//! That holds when the extension and host were built against a layout-compatible
//! `luna-model` *and* a compatible `pyo3`. When they were not, the `unsafe`
//! pointer casts in the sibling `from_capsule` impls reinterpret unrelated
//! memory and the process typically segfaults with no diagnostic.
//!
//! [`CAPSULE_ABI`] guards that boundary. It is a dedicated ABI version,
//! deliberately *decoupled* from the crate release version, so that patch and
//! minor releases which do not change any crossing type keep already-shipped
//! extensions working. Embedding it in every capsule name (see [`capsule_name`])
//! turns a mismatch into a clean Python error at the `PyCapsule_GetPointer`
//! name check instead of a silent segfault, and the host also exports it to
//! Python so extensions can fail fast at import time with a descriptive error.
//!
//! # Bumping rule
//!
//! Increment [`CAPSULE_ABI`] whenever **either** of the following changes:
//!
//! * the in-memory layout of any type passed through a capsule changes — see the
//!   `to_capsule`/`from_capsule` impls under `crates/python/src/ffi/`, or
//! * `pyo3` is upgraded across an ABI-breaking boundary (any major/minor bump).
//!
//! Do **not** bump it for ordinary releases that leave the bridge untouched:
//! that is exactly what keeps `pyo3-lunamodel@x.y.z` extensions compatible with
//! later `luna-model` releases.

use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::sync::{LazyLock, Mutex};

/// Current ABI version of the raw-pointer capsule bridge.
///
/// See the [module documentation](self) for the meaning of this number and the
/// rule for when to bump it.
pub const CAPSULE_ABI: u32 = 1;

/// Resolves the stable per-type capsule base name to the `'static` name used on
/// the wire for the current [`CAPSULE_ABI`].
///
/// `PyCapsule` stores the name pointer without copying it, so the returned name
/// must outlive every capsule created with it — hence `&'static`.
///
/// ABI `1` returns the historical name unchanged, so extensions built before
/// capsule versioning existed (which baked in these exact names) stay
/// compatible. Every later ABI prefixes the base name with the ABI version, so
/// a capsule produced by a mismatched build fails the name check in
/// [`pointer_checked`](pyo3::types::PyCapsuleMethods::pointer_checked) cleanly
/// rather than being reinterpreted.
pub(crate) fn capsule_name(base: &'static CStr) -> &'static CStr {
    let Some(versioned) = versioned_name(CAPSULE_ABI, base) else {
        // ABI 1 fast path: the historical constant is already `'static`.
        return base;
    };
    intern(base, versioned)
}

/// Computes the ABI-prefixed capsule name for `abi`, or `None` when `abi` is the
/// pre-versioning baseline (`1`) and the historical `base` name is used as-is.
///
/// Split out from [`capsule_name`] so the prefixing logic is unit-testable
/// independently of the compiled-in [`CAPSULE_ABI`] and free of global state.
fn versioned_name(abi: u32, base: &CStr) -> Option<CString> {
    // ABI 1 is the pre-versioning baseline: keep the historical names unchanged.
    if abi == 1 {
        return None;
    }
    let base = base.to_str().expect("capsule base names are valid UTF-8");
    Some(
        CString::new(format!("lunamodel.abi{abi}.{base}"))
            .expect("capsule names never contain an interior NUL byte"),
    )
}

/// Interns `versioned` so it can be handed out as `&'static CStr`.
fn intern(base: &'static CStr, versioned: CString) -> &'static CStr {
    static NAMES: LazyLock<Mutex<HashMap<&'static CStr, &'static CStr>>> =
        LazyLock::new(|| Mutex::new(HashMap::new()));
    let mut names = NAMES.lock().expect("capsule name table poisoned");
    names
        .entry(base)
        .or_insert_with(|| Box::leak(versioned.into_boxed_c_str()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abi_1_keeps_historical_names_verbatim() {
        // Backward compatibility contract: at ABI 1 the wire name must equal the
        // historical constant byte-for-byte, otherwise extensions built before
        // capsule versioning would stop interoperating.
        let base = c"builtins.capsule.PyBounds";
        assert_eq!(versioned_name(1, base), None);
    }

    #[test]
    fn later_abi_prefixes_the_base_name() {
        // A bumped ABI must change the wire name so a mismatched extension fails
        // the capsule name check instead of reinterpreting foreign memory.
        let base = c"builtins.capsule.PyBounds";
        assert_eq!(
            versioned_name(2, base).as_deref(),
            Some(c"lunamodel.abi2.builtins.capsule.PyBounds"),
        );
    }

    #[test]
    fn capsule_name_is_stable_across_calls() {
        // Interning must hand out the same pointer for repeated lookups so the
        // name outlives every capsule and is never leaked more than once.
        let base = c"builtins.capsule.PyBounds";
        assert_eq!(capsule_name(base), base);
        assert!(std::ptr::eq(capsule_name(base), capsule_name(base)));
    }
}
