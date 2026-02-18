use crate::{BasePass, Pass, TransformationPass, passes::MaxBiasAnalysis};

#[derive(Debug, Clone)]
pub struct NaiveConstraintsToObjective {
    penalty: f64,
}

impl NaiveConstraintsToObjective {
    pub fn new(penalty: f64) -> Self {
        Self { penalty }
    }
}

impl BasePass for NaiveConstraintsToObjective {
    fn name(&self) -> String {
        String::from("constraints-to-objective")
    }

    fn requires(&self) -> Vec<String> {
        vec![MaxBiasAnalysis::new().name()]
    }
}

impl TransformationPass for NaiveConstraintsToObjective {
    fn run(
        &self,
        model: lunamodel_core::Model,
        cache: &crate::AnalysisCache,
    ) -> crate::TransformationPassResult {
        todo!()
    }

    fn backwards(
        &self,
        solution: lunamodel_core::Solution,
        cache: &crate::AnalysisCache,
    ) -> lunamodel_core::Solution {
        todo!()
    }

    fn invalidates(&self) -> Vec<String> {
        todo!()
    }
}

impl Into<Pass> for NaiveConstraintsToObjective {
    fn into(self) -> Pass {
        Pass::Transformation(Box::new(self))
    }
}
