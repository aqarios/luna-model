use std::fmt::Debug;

use pyo3::{
    prelude::*,
    types::{PyDict, PyTuple},
};

use super::{
    passes::py_pass_base::{PyPass, PyPassPy},
    py_analysis_cache::PyAnalysisCache,
    py_pass_manager::CompilationError,
    py_transformation_pass_result::PyTransformationPassResult,
};
use crate::{
    core::{Model, Solution},
    py_bindings::py_model::PyModel,
    transformations::{
        analysis_cache::AnalysisCache,
        base_passes::{
            BasePass, Pass, TransformationPass, TransformationPassResult, TransformationType,
        },
        errors::TransformationPassError,
    },
};

#[pyclass(unsendable, subclass, name = "TransformationPass")]
#[derive(Clone, Debug)]
pub struct PyTransformationPass {}

#[pymethods]
impl PyTransformationPass {
    #[new]
    #[pyo3(signature=(*args, **kwargs))]
    #[allow(unused_variables)]
    fn py_new(args: &Bound<'_, PyAny>, kwargs: Option<&Bound<'_, PyAny>>) -> Self {
        Self {}
    }

    // #[pyo2(name = "name")]
    // fn py_name(&self) -> PyResult<String> {
    //     Err(pyo3::exceptions::PyNotImplementedError::new_err(
    //         "Must override name",
    //     ))
    // }

    // #[pyo3(name = "requires")]
    // fn py_requires(&self) -> PyResult<&[&str]> {
    //     Err(pyo3::exceptions::PyNotImplementedError::new_err(
    //         "Must override requires",
    //     ))
    // }

    // #[pyo3(name = "run")]
    // fn py_run(
    //     &self,
    //     _model: PyModel,
    //     _cache: &PyAnalysisCache,
    // ) -> PyResult<(PyModel, TransformationType)> {
    //     Err(pyo3::exceptions::PyNotImplementedError::new_err(
    //         "Must override run",
    //     ))
    // }
}

// #[pymethods]
// impl PyTransformationPassAdapter {
//
// }

impl PyPass for Py<PyTransformationPass> {
    fn as_pass(self) -> Pass {
        Pass::Transformation(Box::new(PyTransformationPassAdapter::new(self)))
    }
}

pub struct PyTransformationPassAdapter {
    inner: Py<PyTransformationPass>,
}

impl PyTransformationPassAdapter {
    fn new(inner: Py<PyTransformationPass>) -> Self {
        Self { inner }
    }
}

impl PyTransformationPassAdapter {}

impl BasePass for PyTransformationPassAdapter {
    fn name(&self) -> String {
        Python::with_gil(|py| {
            self.inner
                .getattr(py, "name")
                .and_then(|res| res.extract::<String>(py))
                .expect("no 'name' metod")
        })
    }

    fn requires(&self) -> Vec<String> {
        Python::with_gil(|py| {
            self.inner
                .getattr(py, "requires")
                .and_then(|res| res.extract::<Vec<String>>(py))
                .expect("no 'requires' method")
        })
    }
}

impl TransformationPass for PyTransformationPassAdapter {
    fn invalidates(&self) -> &[&str] {
        &[]
    }

    fn run(&self, mut model: Model, cache: &AnalysisCache) -> TransformationPassResult {
        let res = Python::with_gil(|py| {
            let some = self
                .inner
                .call_method1(
                    py,
                    "run",
                    (
                        PyModel::new(model),
                        PyAnalysisCache::new(AnalysisCache::new()), // TODO: fix this
                    ),
                )
                .unwrap();
            let (py_model, py_tt): (Py<PyModel>, Py<TransformationType>) =
                some.extract(py).unwrap();
            let py_model_borrow = py_model.borrow(py);
            let pymodel = py_model_borrow.clone();
            model = pymodel.concrete_model.borrow().clone();
            let tt = py_tt.borrow(py);
            (model, tt.clone())
        });
        Ok(res)
    }

    fn backwards(&self, mut solution: Solution, _cache: &AnalysisCache) -> Solution {
        todo!()
    }
}

impl Debug for PyTransformationPassAdapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

// impl Clone for PyTransformationPassAdapter {
//     fn clone(&self) -> Self {
//         todo!()
//     }
// }
