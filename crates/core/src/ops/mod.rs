//! Internal operator implementations and fallible arithmetic traits.
//!
//! This module is where LunaModel wires its algebraic surface area onto Rust's
//! operator ecosystem. Public users usually encounter the behavior indirectly
//! through `Expression`, `VarRef`, `Constraint`, and `Bounds` methods. Internal
//! developers working on new algebraic types or Python bindings tend to end up
//! here because this is the layer that decides:
//!
//! - which owned/borrowed combinations are supported,
//! - where environment compatibility is validated,
//! - how low-level multiplication fragments are reduced before they become full
//!   [`Expression`](crate::Expression) values, and
//! - which operations may fail and therefore need custom `Lm*` traits instead
//!   of the infallible standard-library traits alone.
//!
//! Most of the heavy lifting lives in the submodules:
//!
//! - `expr` for full expression arithmetic,
//! - `vref` for variable-reference operations,
//! - `term` for sparse term storage multiplication helpers,
//! - `constraint` for constraint evaluation,
//! - `bounds` for bounds evaluation, and
//! - `mcrs` for the macro layer that expands the repetitive owned/borrowed
//!   operator impl matrix.
mod mcrs;
mod utils;

mod bounds;
mod constraint;
mod expr;
mod term;
mod traits;
mod vref;

pub use traits::{LmAddAssign, LmMulAssign, LmPow, LmPowAssign, LmSubAssign};

pub use utils::{Lookup, SolutionLookup};
