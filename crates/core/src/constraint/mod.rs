//! Constraint types.
//!
//! Constraints pair a symbolic left-hand side expression with a numeric
//! right-hand side and a comparator. They are stored inside
//! [`ConstraintCollection`] so models can preserve insertion order and provide
//! stable, name-based lookup.

mod collection;
mod constr;

/// Named constraint collection used by [`crate::Model`].
pub use collection::ConstraintCollection;
/// Single algebraic constraint.
pub use constr::Constraint;
