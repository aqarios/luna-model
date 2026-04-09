use lunamodel_transformv2::transformation::EqualityConstraintsToQuadraticPenaltyPass;
use pyo3::{pyclass, pymethods};

#[pyclass(subclass)]
pub struct PyEqualityConstraintsToQuadraticPenaltyPass(
    pub EqualityConstraintsToQuadraticPenaltyPass,
);

#[pymethods]
impl PyEqualityConstraintsToQuadraticPenaltyPass {
    #[new]
    fn new(penalty_scaling: f64) -> Self {
        Self(EqualityConstraintsToQuadraticPenaltyPass::new(
            penalty_scaling,
        ))
    }
}

impl PyEqualityConstraintsToQuadraticPenaltyPass {
    pub fn to_rs(&self) -> EqualityConstraintsToQuadraticPenaltyPass {
        self.0.clone()
    }
}
