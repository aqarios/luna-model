mod context;
mod entry;
mod envelope;
mod manager;
mod output;
mod pass;
mod pipeline;
mod record;
mod utils;

mod adapter;
pub mod builtin;

pub use adapter::{
    PyAnalysisPass, PyAnalysisPassAdapter, PyControlFlowPass, PyControlFlowPassAdapter,
    PyControlFlowPlan, PyTransformationPass, PyTransformationPassAdapter,
};
pub use context::PyPassContext;
pub use manager::PyPassManager;
pub use output::PyTransformationOutput;
pub use pipeline::PyPipeline;
pub use record::PyTransformationRecord;
pub use entry::PyPassEntry;

pub fn register_backward() {
    lunamodel_transformv2::register_backward();
    lunamodel_transpiler::register_backward::<PyTransformationPassAdapter>();
}
