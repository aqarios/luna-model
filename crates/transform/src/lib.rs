//! Built-in model analyses, transformations, control-flow passes, and pipelines.
//!
//! This crate contains LunaModel's standard library of passes built on top of
//! `lunamodel-transpiler`. The implementations here are domain-aware: they know
//! about model structure, variable domains, constraint normalization, and the
//! reversible artifacts needed for backward execution.
mod error;

pub mod analysis;
pub mod composite;
pub mod control_flow;
pub mod pipelines;
pub mod transformation;

mod utils;

pub fn register_backward() {
    transformation::register_backward();
    composite::register_backward();
}
