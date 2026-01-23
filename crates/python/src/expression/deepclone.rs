use lunamodel_core::Expression;
use lunamodel_unwind::*;
use pyo3::pymethods;

use super::PyExpression;

#[unwindable]
#[pymethods]
impl PyExpression {
    #[staticmethod]
    fn deep_clone_many(exprs: Vec<PyExpression>) -> Vec<PyExpression> {
        let mapped: Vec<Expression> = exprs.into_iter().map(|pye| pye.into()).collect();
        let m2: Vec<&Expression> = mapped.iter().collect();
        let cloned = Expression::deep_clone_many(m2.as_slice());
        cloned.into_iter().map(|e| PyExpression::new(e)).collect()
    }
}
