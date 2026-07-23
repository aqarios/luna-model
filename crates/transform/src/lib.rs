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

/// Registers on-demand serializers for built-in analysis results so plugins can consume them across
/// the C-ABI. Call once at host startup.
pub fn register_builtins() {
    lunamodel_transpiler::register_analysis_serializer(
        <analysis::MinValueForConstraintAnalysis as lunamodel_transpiler::AnalysisPass>::PROVIDES,
        lunamodel_capi_plugin_proto::proto::MinConstraintValues::TYPE_TAG,
        analysis::min_val_in_constr::serialize_min_constraint_values,
    );
}
