//! Core symbolic modeling primitives for LunaModel.
//!
//! This crate owns the central in-memory representation of an optimization
//! problem:
//!
//! - [`Environment`] stores variables and their metadata.
//! - [`Expression`] represents symbolic algebra over variables.
//! - [`Constraint`] and [`ConstraintCollection`] constrain expressions.
//! - [`Model`] combines an objective, constraints, and an optimization sense.
//! - [`Solution`] and its related types represent solver outputs and samples.
//!
//! Most higher-level crates in the workspace either translate into these types,
//! transform them, or expose them through bindings. If you are new to the
//! project, this crate is the best place to build a mental model of how the
//! rest of the workspace fits together.
pub mod prelude;

mod bounds;
mod constraint;
mod environment;
mod expression;
mod model;
pub mod ops;
pub mod solution;
mod traits;
mod utils;
mod variable;

pub use constraint::{Constraint, ConstraintCollection};
pub use environment::{ArcEnv, Environment};
pub use expression::Expression;
pub use model::Model;
pub use solution::{Solution, Timer, Timing, ValueSource};

/// Trait for indexing domain collections with fallible APIs.
pub use traits::TryIndex;
