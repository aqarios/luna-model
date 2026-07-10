//! Python wrappers for the transpiler/transform pipeline system.
mod context;
mod entry;
mod envelope;
mod manager;
mod output;
mod pass;
mod pipeline;
mod record;
mod utils;
mod error;

mod adapter;
pub mod builtin;

pub use adapter::{
    PyAnalysisPass, PyAnalysisPassAdapter, PyCompositePass, PyCompositePassAdapter,
    PyControlFlowPass, PyControlFlowPassAdapter, PyControlFlowPlan, PyMetaAnalysisPass,
    PyMetaAnalysisPassAdapter, PyTransformationPass, PyTransformationPassAdapter,
};
pub use context::PyPassContext;
pub use entry::PyPassEntry;
pub use manager::PyPassManager;
pub use output::PyTransformationOutput;
pub use pipeline::PyPipeline;
pub use record::PyTransformationRecord;

/// Registers built-in and Python-adapter backward handlers.
pub fn register_backward() {
    lunamodel_transform::register_backward();
    lunamodel_transpiler::register_backward::<PyTransformationPassAdapter>();
    lunamodel_transpiler::register_backward::<PyCompositePassAdapter>();
}
