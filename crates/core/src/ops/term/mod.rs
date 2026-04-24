//! Internal multiplication helpers for term storage.
//!
//! These modules implement the lower-level `PrvMul` combinations used by the
//! expression multiplication code. They are intentionally not part of the
//! user-facing API; their job is to map combinations of sparse term storages
//! into [`crate::ops::utils::VarMulRes`] fragments that can later be merged into
//! a full expression.

mod higher_order;
mod linear;
mod quadratic;
