//! Commonly used core types and traits for crate users and internal modules.
//!
//! The prelude intentionally re-exports the modeling primitives that appear in
//! most workflows so call sites can stay focused on the actual modeling logic
//! instead of long import lists.

pub use crate::bounds::{Bounds, LazyBounds};
pub use crate::constraint::{Constraint, ConstraintCollection};
pub use crate::environment::{ArcEnv, Environment};
pub use crate::expression::Expression;
pub use crate::expression::term::{HigherOrder, Linear, Quadratic};
pub use crate::model::Model;
pub use crate::ops::{LmAddAssign, LmMulAssign, LmPow, LmPowAssign, LmSubAssign};
pub use crate::solution::{
    Solution,
    timing::{Timer, Timing},
};
pub use crate::traits::ContentEquality;
pub use crate::traits::{DefaultEditable, Editable};
pub use crate::variable::{VarRef, Variable};
