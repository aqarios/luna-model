//! Macro helpers for generating the owned/borrowed operator matrix.
//!
//! The core algebra types intentionally support many combinations such as
//! `owned + borrowed`, `borrowed * owned`, and the corresponding `*_assign`
//! variants. Writing all of those impls by hand would be repetitive and easy to
//! drift out of sync, so these macros generate the boilerplate around the
//! fallible `Lm*Assign` traits.
mod add;
mod mul;
mod sub;
