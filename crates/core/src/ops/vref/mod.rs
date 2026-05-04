//! Variable-reference level operators.
//!
//! `VarRef` is the smallest user-facing algebraic building block in the core
//! crate. These impls define how a single environment-bound variable behaves
//! under arithmetic before the result is widened into a full
//! [`Expression`](crate::Expression).
mod add;
mod eval;
mod mul;
mod neg;
mod not;
mod pow;
mod sub;
