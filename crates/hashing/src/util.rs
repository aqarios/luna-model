//! Small shared helpers for streaming content into a [`Hasher`].

use std::hash::Hasher;

/// Length-prefixes a variable-length byte sequence before writing it.
///
/// Without a length prefix, two adjacent variable-length fields could be
/// reinterpreted as a different split of the same underlying bytes (e.g.
/// `"ab"` followed by `"c"` hashing identically to `"a"` followed by
/// `"bc"`). Prefixing each one with its own length rules that out.
pub fn write_bytes(h: &mut impl Hasher, bytes: &[u8]) {
    h.write_u64(bytes.len() as u64);
    h.write(bytes);
}
