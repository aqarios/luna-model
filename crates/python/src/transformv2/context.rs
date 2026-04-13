use lunamodel_transformv2::analysis::{
    CheckModelSpecsAnalysis, MaxBias, MaxBiasAnalysis, MinConstraintValues,
    MinValueForConstraintAnalysis, SpecsAnalysis,
};
use lunamodel_transpiler::{AnalysisKey, AnalysisManager, AnalysisPass, PassContext};
use lunamodel_types::Specs;
use pyo3::{IntoPyObjectExt, Py, PyAny, PyResult, Python, pyclass, pymethods};

use crate::{
    PyModelSpecs,
    transformv2::{
        adapter::PyAnalysisPassAdapterResult,
        builtin::analysis::{PyMaxBias, PyMinConstraintValues},
    },
};

#[pyclass(subclass)]
pub struct PyPassContext {
    manager: AnalysisManager,
}

impl From<AnalysisManager> for PyPassContext {
    fn from(manager: AnalysisManager) -> Self {
        Self { manager }
    }
}

impl<'c> Into<PassContext<'c>> for &'c PyPassContext {
    fn into(self) -> PassContext<'c> {
        PassContext::new(&self.manager)
    }
}

#[pymethods]
impl PyPassContext {
    #[new]
    fn new() -> Self {
        Self {
            manager: AnalysisManager::default(),
        }
    }

    fn require_analysis(&self, py: Python, key: String) -> PyResult<Py<PyAny>> {
        let res = match key.as_str() {
            x if x == CheckModelSpecsAnalysis::PROVIDES => {
                let _: &() = self.manager.require(&CheckModelSpecsAnalysis::key())?;
                &py.None()
            }
            x if x == MaxBiasAnalysis::PROVIDES => {
                let a: &MaxBias = self.manager.require(&MaxBiasAnalysis::key())?;
                &PyMaxBias(*a).into_py_any(py)?
            }
            x if x == MinValueForConstraintAnalysis::PROVIDES => {
                let a: &MinConstraintValues = self
                    .manager
                    .require(&MinValueForConstraintAnalysis::key())?;
                &PyMinConstraintValues(a.clone()).into_py_any(py)?
            }
            x if x == SpecsAnalysis::PROVIDES => {
                let a: &Specs = self.manager.require(&SpecsAnalysis::key())?;
                &PyModelSpecs::from(a.clone()).into_py_any(py)?
            }
            _ => {
                &self
                    .manager
                    .require(&AnalysisKey::<&PyAnalysisPassAdapterResult>::new(key))?
                    .0
            }
        };
        Ok(res.clone_ref(py))
    }
}
