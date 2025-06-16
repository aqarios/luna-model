use crate::{
    core::{
        expression::{BiasConstraints, IndexConstraints},
        operations::MulAssignToExpression,
        solution::AssignmentBaseTypes,
        Model, Sense, Solution,
    },
    transformations::{
        analysis_cache::AnalysisCache,
        base_passes::{BasePass, TransformationPass, TransformationPassResult, TransformationType},
    },
};

#[derive(Debug, Clone)]
pub struct ChangeSensePass {
    pub sense: Sense,
}

impl BasePass for ChangeSensePass {
    fn name(&self) -> &str {
        "change-sense"
    }

    fn requires(&self) -> &[&str] {
        &[]
    }
}

impl<Index: IndexConstraints, Bias: BiasConstraints, AssignmentTypes: AssignmentBaseTypes>
    TransformationPass<Index, Bias, AssignmentTypes> for ChangeSensePass
{
    fn invalidates(&self) -> &[&str] {
        &[]
    }

    fn run(
        &self,
        mut model: Model<Index, Bias>,
        _cache: &AnalysisCache,
    ) -> TransformationPassResult<Index, Bias> {
        if model.sense == self.sense {
            return Ok((model, TransformationType::NoTranform));
        } else {
            model.objective.borrow_mut().mul_assign(-Bias::one());
            model.set_sense(self.sense);
            return Ok((model, TransformationType::DidTransform));
        }
    }

    fn backwards(
        &self,
        mut solution: Solution<Bias, AssignmentTypes>,
        _cache: &AnalysisCache,
    ) -> Solution<Bias, AssignmentTypes> {
        solution.obj_values = solution
            .obj_values
            .into_iter()
            .map(|x| x.map(|y| y * (-1.0)))
            .collect();
        solution
    }
}
