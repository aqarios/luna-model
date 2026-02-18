mod unicode;

mod base;
mod cache;
mod execution;
mod ir;
mod log;
mod pass_manager;

pub mod passes;
pub mod pipelines;
pub use base::Pass;
pub use base::{
    ActionType, AnalysisPass, AnalysisPassResult, BasePass, TransformationOutcome,
    TransformationPass, TransformationPassResult,
};
pub use cache::{AnalysisCache, AnalysisCacheElement};
pub use ir::IR;
pub use log::{ExecutionLog, LogElement};
pub use pass_manager::PassManager;

// pub use base::AsPyPass;

// #[cfg(feature = "py")]
// pub mod py;
//
// use lunamodel_tpass::register_pytransformations;

// #[cfg(feature = "py")]
// register_pytransformations!(
//     specials = {PyAnalysisPass, PyTransformationPass, PyPipeline, PyMetaAnalysisPass},
//     extras = {PyAnalysisCache, PyPassManager, ActionType, MaxBias, PyIR, PyLogElement, BinarySpinInfo, StructuredPyTransformationOutcome, PyBasePass},
//     passes = {
//         PyChangeSensePass, PyMaxBiasAnalysis, PyBinarySpinPass, PyIfElsePass
//     },
// );
