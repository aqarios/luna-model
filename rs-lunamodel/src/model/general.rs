use crate::{
    ConstraintCollection, Expression, Model, Result, ResultView, SampleView, Solution, Specs,
    VarRef,
};

impl Model {
    /// Retains the same synchronized host model.
    ///
    /// Unlike `try_clone`, mutations through either handle remain mutually visible.
    pub fn try_retain(&self) -> Result<Self> {
        out!(model_retain(@self))
    }

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

    /// Substitutes every `(target, replacement)` pair in one walk over the
    /// objective and the constraints, and removes every target from the
    /// environment. No replacement may contain a target and no target may
    /// appear twice; otherwise the model is left unchanged.
    pub fn substitute_many(&mut self, subs: &[(&VarRef, &Expression)]) -> Result<()> {
        require_api!(model_substitute_many);
        let targets: Vec<_> = subs.iter().map(|(t, _)| t.as_ptr()).collect();
        let replacements: Vec<_> = subs.iter().map(|(_, r)| r.as_ptr()).collect();
        check!(model_substitute_many(
            @mut self,
            targets.as_ptr(),
            replacements.as_ptr(),
            subs.len()
        ))
    }

    /// Evaluates the model against a single sample row, returning that row's
    /// enriched result view.
    pub fn evaluate_sample(&self, sample: &SampleView, tol: Option<f64>) -> Result<ResultView> {
        out!(model_evaluate_sample(@self, @sample, ?tol))
    }

    /// Returns the constraints violated by a single sample row.
    pub fn violated_constraints(
        &self,
        sample: &SampleView,
        tol: Option<f64>,
    ) -> Result<ConstraintCollection> {
        out!(model_violated_constraints(@self, @sample, ?tol))
    }
}
