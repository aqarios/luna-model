//! Bound types for variables.
//!
//! LunaModel distinguishes between eagerly materialized bounds ([`Bounds`]) and
//! bounds that still need to be resolved against a variable type
//! ([`LazyBounds`]). That split lets call sites stay ergonomic while keeping the
//! actual invariants centralized in the concretization logic.

mod concrete;
mod lazy;

/// Concrete lower and upper bounds stored on variables.
pub use concrete::Bounds;
/// Trait for turning deferred bound specifications into concrete bounds.
pub use lazy::Concretize;
/// Deferred bound specification used at construction time.
pub use lazy::LazyBounds;
