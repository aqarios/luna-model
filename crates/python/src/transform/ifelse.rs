use lunamodel_error::LunaModelError;
use lunamodel_transform::{
    BasePass,
    passes::special::{Condition, IfElsePass},
};
use pyo3::{
    Py, PyAny, PyResult, Python, exceptions::PyTypeError, pyclass, pymethods, types::PyAnyMethods,
};

use crate::transform::{PyAnalysisCache, PyPipeline, adapters::PyPipelineAdapter};

#[pyclass(subclass, unsendable)]
#[derive(Debug, Clone)]
pub struct PyIfElsePass(pub IfElsePass);

#[pymethods]
impl PyIfElsePass {
    #[new]
    #[pyo3(signature = (requires, condition, then, otherwise, name=None))]
    fn new(
        requires: Vec<String>,
        condition: Py<PyAny>,
        then: Py<PyPipeline>,
        otherwise: Py<PyPipeline>,
        name: Option<String>,
    ) -> PyResult<Self> {
        Python::attach(|py| {
            if !condition.bind(py).is_callable() {
                Err(PyTypeError::new_err(
                    "The parameter 'condition' must be a callable",
                ))
            } else {
                Ok(())
            }
        })?;

        let pycondition = PyCondition(condition);
        Ok(Self(IfElsePass::new(
            requires,
            Box::new(pycondition),
            Box::new(PyPipelineAdapter::new(then)?),
            Box::new(PyPipelineAdapter::new(otherwise)?),
            name,
        )))
    }
}

impl BasePass for PyIfElsePass {
    fn name(&self) -> String {
        self.0.name()
    }

    fn requires(&self) -> Vec<String> {
        self.0.requires()
    }
}

#[derive(Debug)]
struct PyCondition(pub Py<PyAny>);

impl Condition for PyCondition {
    fn call(
        &self,
        cache: &lunamodel_transform::AnalysisCache,
    ) -> lunamodel_error::LunaModelResult<bool> {
        Python::attach(|py| {
            let pyc: PyAnalysisCache = cache.clone().into();
            let r = self
                .0
                .call1(py, (pyc,))
                .map_err(|e| LunaModelError::Computation(e.to_string().into()))?
                .extract::<bool>(py)
                .map_err(|e| LunaModelError::Computation(e.to_string().into()))?;
            Ok(r)
        })
    }
}

impl Clone for PyCondition {
    fn clone(&self) -> Self {
        Python::attach(|py| PyCondition(self.0.clone_ref(py)))
    }
}
