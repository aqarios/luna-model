//! Term storage for [`crate::Expression`].
//!
//! The expression layer is intentionally split by degree:
//!
//! - [`Linear`] stores one-variable contributions
//! - [`Quadratic`] stores symmetric two-variable contributions
//! - [`HigherOrder`] stores arbitrary-degree contributions
//!
//! The supporting `types` module contains the small internal storage building
//! blocks shared by the linear and quadratic representations.

mod higher_order;
mod linear;
mod quadratic;
pub mod types;

/// Storage for higher-order expression terms.
pub use higher_order::HigherOrder;
/// Storage for linear expression terms.
pub use linear::Linear;
/// Storage for quadratic expression terms.
pub use quadratic::Quadratic;
