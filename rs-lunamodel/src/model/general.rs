use crate::{ConstraintCollection, Expression, Model, Result, SampleView, Solution, Specs, VarRef};

impl Model {
    pub fn satisfies(&self, specs: &Specs) -> Result<bool> {
        out!(model_satisfies(@self, @specs))
    }

    pub fn evaluate_solution(&self, sol: &Solution) -> Result<Solution> {
        out!(model_evaluate_solution(@self, @sol))
    }

    pub fn evaluate_solution_with_tol(&self, sol: &Solution, tol: Option<f64>) -> Result<Solution> {
        out!(model_evaluate_solution_with_tol(@self, @sol, ?tol))
    }

    pub fn substitute(&mut self, target: &VarRef, replacement: &Expression) -> Result<()> {
        check!(model_substitute(@mut self, @target, @replacement))
    }

    pub fn violated_constraints(
        &self,
        sample: &SampleView,
        tol: Option<f64>,
    ) -> Result<ConstraintCollection> {
        out!(model_violated_constraints(@self, @sample, ?tol))
    }
}
