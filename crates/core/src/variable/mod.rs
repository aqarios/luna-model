//! Variable types and references.
//!
//! [`Variable`] stores the metadata owned by an environment, while [`VarRef`]
//! is the cheap handle that expressions and models pass around. Most user-facing
//! APIs work with `VarRef` because it keeps the variable identity plus the
//! environment needed to resolve metadata lazily.

mod var;
mod vref;

/// Owned variable metadata stored inside an environment.
pub use var::Variable;
/// Cheap reference to a variable stored in an environment.
pub use vref::VarRef;
