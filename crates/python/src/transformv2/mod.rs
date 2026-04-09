mod analysis;
mod context;
mod entry;
mod manager;
mod output;
mod pass;
mod record;
mod envelope;

mod adapter;
pub mod builtin;

pub use adapter::{PyTransformationPass, PyTransformationPassAdapter};
pub use context::PyPassContext;
pub use manager::PyPassManager;
pub use output::PyTransformationOutput;
pub use record::PyTransformationRecord;

pub fn register_backward() {
    lunamodel_transformv2::register_backward();
    lunamodel_transpiler::register_backward::<PyTransformationPassAdapter>();
}
