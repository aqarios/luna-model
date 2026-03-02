use lunamodel_transform::{
    AnalysisCache, AnalysisCacheElement,
    passes::{
        BinarySpinInfo, IntegerToBinaryInfo, MaxBias, analysis::MinConstraintValues,
        special::IfElseInfo,
    },
};
use lunamodel_unwind::*;
use pyo3::{IntoPyObjectExt, Py, PyAny, PyResult, Python, pyclass, pymethods};

use crate::PyModelSpecs;

#[pyclass]
#[derive(Clone, Debug)]
pub struct PyAnalysisCache {
    pub c: AnalysisCache,
}

impl From<AnalysisCache> for PyAnalysisCache {
    fn from(c: AnalysisCache) -> Self {
        Self { c }
    }
}

#[unwindable]
#[pymethods]
impl PyAnalysisCache {
    #[new]
    fn new() -> Self {
        Self {
            c: AnalysisCache::new(),
        }
    }

    fn __getitem__(&self, py: Python, key: String) -> PyResult<Option<Py<PyAny>>> {
        if let Some(val) = self.c.get(&key) {
            Ok(Some(val.into_py_any(py)?))
        } else {
            Ok(None)
        }
    }
}

pub trait AnalysisCacheElementPyMethods {
    fn into_py_any(&self, py: Python) -> PyResult<Py<PyAny>>;
    fn specific_or_else_any(py: Python, any: Py<PyAny>) -> Self;
}

impl AnalysisCacheElementPyMethods for AnalysisCacheElement {
    fn into_py_any(&self, py: Python) -> PyResult<Py<PyAny>> {
        Ok(match self {
            AnalysisCacheElement::MaxBiasAnalysis(v) => v.clone().into_py_any(py)?,
            AnalysisCacheElement::BinarySpinInfoAnalysis(v) => v.clone().into_py_any(py)?,
            AnalysisCacheElement::IfElseInfoAnalysis(v) => v.clone().into_py_any(py)?,
            AnalysisCacheElement::MinValueInConstraintAnalysis(v) => v.clone().into_py_any(py)?,
            AnalysisCacheElement::SpecsAnalysis(v) => {
                PyModelSpecs::from(v.clone()).into_py_any(py)?
            }
            AnalysisCacheElement::IntegerToBinaryInfoAnalysis(v) => v.clone().into_py_any(py)?,
            AnalysisCacheElement::General(v) => v.clone().into_py_any(py)?,
            AnalysisCacheElement::PyAnalysis(v) => v.into_py_any(py)?,
        })
    }

    fn specific_or_else_any(py: Python, any: Py<PyAny>) -> Self {
        if let Ok(mb) = any.extract::<MaxBias>(py) {
            AnalysisCacheElement::MaxBiasAnalysis(mb)
        } else if let Ok(bsi) = any.extract::<BinarySpinInfo>(py) {
            AnalysisCacheElement::BinarySpinInfoAnalysis(bsi)
        } else if let Ok(iei) = any.extract::<IfElseInfo>(py) {
            AnalysisCacheElement::IfElseInfoAnalysis(iei)
        } else if let Ok(mcv) = any.extract::<MinConstraintValues>(py) {
            AnalysisCacheElement::MinValueInConstraintAnalysis(mcv)
        } else if let Ok(s) = any.extract::<PyModelSpecs>(py) {
            AnalysisCacheElement::SpecsAnalysis(s.s)
        } else if let Ok(itbi) = any.extract::<IntegerToBinaryInfo>(py) {
            AnalysisCacheElement::IntegerToBinaryInfoAnalysis(itbi)
        } else if let Ok(v) = any.extract::<Vec<String>>(py) {
            AnalysisCacheElement::General(v)
        } else {
            AnalysisCacheElement::PyAnalysis(any)
        }
    }
}
