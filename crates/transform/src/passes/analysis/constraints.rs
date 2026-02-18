use crate::{AnalysisPass, BasePass, Pass};

#[derive(Debug, Clone)]
pub struct ConstraintsKindAnalysis;

impl ConstraintsKindAnalysis {
    pub fn new() -> Self {
        Self {}
    }
}

impl BasePass for ConstraintsKindAnalysis {
    fn name(&self) -> String {
        String::from("constraint-analysis")
    }
}

#[cfg_attr(feature = "py", pyo3::pyclass(get_all))]
#[derive(Debug, Clone, Copy)]
pub struct ConstraintsKind {
    // pub ctypes: ,
}

impl AnalysisPass for ConstraintsKindAnalysis {
    fn run(
        &self,
        model: &lunamodel_core::Model,
        cache: &crate::AnalysisCache,
    ) -> crate::AnalysisPassResult {
        todo!()
    }
}

impl Into<Pass> for ConstraintsKindAnalysis {
    fn into(self) -> Pass {
        Pass::Analysis(Box::new(self))
    }
}
