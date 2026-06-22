//! Shared helpers for transform passes.

use rand::distr::{Alphanumeric, Distribution};

/// Length of the random alphanumeric suffix appended to generated names.
///
/// 10 characters over `[0-9A-Za-z]` (~59 bits) make collisions negligible for
/// any realistic number of passes; raise it for stronger guarantees or lower it
/// for shorter names.
const SUFFIX_LEN: usize = 10;

/// Generates a unique, collision-resistant name with the given prefix.
///
/// Auto-generated names exist only to avoid clashes — pass an explicit name
/// (e.g. to [`crate::control_flow::IfElsePass::new`]) when you want a stable,
/// meaningful one.
pub fn unique_name(prefix: &str) -> String {
    let suffix: String = Alphanumeric
        .sample_iter(rand::rng())
        .take(SUFFIX_LEN)
        .map(char::from)
        .collect();
    format!("{prefix}-{suffix}")
}
