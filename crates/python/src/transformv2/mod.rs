mod context;
mod entry;
mod envelope;
mod manager;
mod output;
mod pass;
mod record;

mod adapter;
pub mod builtin;

pub use adapter::{
    PyAnalysisPass, PyAnalysisPassAdapter, PyTransformationPass, PyTransformationPassAdapter,
};
pub use context::PyPassContext;
pub use manager::PyPassManager;
pub use output::PyTransformationOutput;
pub use record::PyTransformationRecord;

pub fn register_backward() {
    lunamodel_transformv2::register_backward();
    lunamodel_transpiler::register_backward::<PyTransformationPassAdapter>();
}
