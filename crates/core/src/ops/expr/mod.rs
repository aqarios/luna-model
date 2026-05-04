//! Expression operations.
//!
//! These modules implement the algebra that turns [`crate::Expression`] into a
//! usable symbolic type: arithmetic, evaluation, substitution, and structural
//! rewrites.

mod add;
mod mul;
mod neg;
mod pow;
mod sub;

mod eval;
mod separate;
mod substitute;
