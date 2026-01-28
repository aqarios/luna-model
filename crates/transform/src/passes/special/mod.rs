mod abstract_pipeline;
mod ifelse;
mod meta_analysis;
mod pipeline;

pub use abstract_pipeline::AbstractPipeline;
pub use ifelse::{IfElseInfo, IfElsePass, Condition};
pub use meta_analysis::{MetaAnalysisPass, MetaAnalysisPassResult};
pub use pipeline::{Pipeline, PipelineResult};
