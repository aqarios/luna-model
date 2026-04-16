use std::sync::Arc;

use lunamodel_core::{Model, Solution};
use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_transpiler::{AnalysisKey, CompositePass, PassContext, Reversible};
use pyo3::{
    Py, PyAny, PyErr, Python,
    types::{PyAnyMethods, PyModule, PyTuple, PyTupleMethods},
};

use crate::{
    PyModel, PySolution,
    transform::{
        PyPassContext,
        adapter::{
            PyAnalysisPassAdapterResult, PyTransformationPassAdapterArtifact,
            transformation::envelope::BackwardEnvelope,
        },
        utils::map_pyerr,
    },
};

use super::PyCompositePass;

pub struct PyCompositePassAdapter {
    inner: Py<PyCompositePass>,
    name: String,
    requires: Vec<String>,
    provides: String,
    invalidates: Vec<String>,
}

impl PyCompositePassAdapter {
    pub fn new(py: Python, inner: Py<PyCompositePass>) -> LunaModelResult<Self> {
        let name: String = inner
            .call_method0(py, "name")
            .map_err(map_pyerr)?
            .extract(py)
            .map_err(map_pyerr)?;
        let requires: Vec<String> = inner
            .call_method0(py, "requires")
            .map_err(map_pyerr)?
            .extract(py)
            .map_err(map_pyerr)?;
        let provides: String = inner
            .call_method0(py, "provides")
            .map_err(map_pyerr)?
            .extract(py)
            .map_err(map_pyerr)?;
        let invalidates: Vec<String> = inner
            .call_method0(py, "invalidates")
            .map_err(map_pyerr)?
            .extract(py)
            .map_err(map_pyerr)?;

        Ok(Self {
            inner,
            name,
            requires,
            provides,
            invalidates,
        })
    }

    pub fn inner(&self, py: Python) -> Py<PyCompositePass> {
        self.inner.clone_ref(py)
    }
}

impl CompositePass for PyCompositePassAdapter {
    type Result = PyAnalysisPassAdapterResult;

    const NAME: &'static str = "lunamodel::PyCompositePassAdapter";
    const PROVIDES: &'static str = "lunamodel::PyCompositeProvided";

    fn name(&self) -> &str {
        &self.name
    }

    fn provides(&self) -> &str {
        &self.provides
    }

    fn invalidates(&self) -> &[String] {
        &self.invalidates
    }

    fn requires(&self) -> &[String] {
        &self.requires
    }

    fn forward(
        &self,
        model: &mut Model,
        ctx: &PassContext,
    ) -> LunaModelResult<(Self::Artifact, Self::Result)> {
        let (py_model, py_artifact, py_result, backward_envelope) = Python::attach(|py| {
            let obj = self.inner.bind(py);

            // FOR BACKWARD
            let cls = obj.getattr("__class__").map_err(map_pyerr)?;
            let module: String = cls
                .getattr("__module__")
                .map_err(map_pyerr)?
                .extract()
                .map_err(PyErr::from)
                .map_err(map_pyerr)?;

            let qualname: String = cls
                .getattr("__qualname__")
                .map_err(map_pyerr)?
                .extract()
                .map_err(PyErr::from)
                .map_err(map_pyerr)?;
            // FOR BACKWARD

            let res = obj
                .call_method1(
                    "_forward",
                    (
                        PyModel::from(model.clone()),
                        PyPassContext::from(ctx.manager().clone()),
                    ),
                )
                .map_err(map_pyerr)?;
            let outcome: Py<PyTuple> = res.extract().map_err(|e| map_pyerr(PyErr::from(e)))?;
            let t = outcome.bind(py);
            if t.len() != 3 {
                return Err(LunaModelError::Internal(
                    "expected (model, artifact, result)".into(),
                ));
            }
            let py_model: PyModel = t
                .get_item(0)
                .map_err(map_pyerr)?
                .extract()
                .map_err(PyErr::from)
                .map_err(map_pyerr)?;

            let py_artifact_obj: Py<PyAny> = t
                .get_item(1)
                .map_err(map_pyerr)?
                .extract()
                .map_err(PyErr::from)
                .map_err(map_pyerr)?;
            let py_result_obj: Py<PyAny> = t
                .get_item(2)
                .map_err(map_pyerr)?
                .extract()
                .map_err(PyErr::from)
                .map_err(map_pyerr)?;

            Ok((
                py_model,
                py_artifact_obj,
                py_result_obj,
                BackwardEnvelope { module, qualname },
            ))
        })?;
        *model = Arc::into_inner(py_model.0.m)
            .ok_or(LunaModelError::Internal(
                "Model reference leaked out of forward scope.".into(),
            ))?
            .into_inner();
        let artifact = PyTransformationPassAdapterArtifact {
            artifact: py_artifact,
            backward: backward_envelope,
        };
        let result = PyAnalysisPassAdapterResult(py_result);
        Ok((artifact, result))
    }

    fn key<T>() -> AnalysisKey<T> {
        unimplemented!(
            "the key on the PyCompositePassAdapter is not stable and should not be called."
        )
    }
}

impl Reversible for PyCompositePassAdapter {
    type Artifact = PyTransformationPassAdapterArtifact;

    const ID: &'static str = "lunamodel::PyCompositePassAdapter";

    fn backward(artifact: &Self::Artifact, solution: Solution) -> LunaModelResult<Solution> {
        let PyTransformationPassAdapterArtifact { artifact, backward } = artifact;
        let py_sol: PySolution = Python::attach(|py| {
            let module = PyModule::import(py, &backward.module).map_err(map_pyerr)?;
            let cls = module.getattr(&backward.qualname).map_err(map_pyerr)?;
            let obj = cls
                .call_method1("_backward", (artifact.bind(py), PySolution::from(solution)))
                .map_err(map_pyerr)?;
            obj.extract::<PySolution>()
                .map_err(PyErr::from)
                .map_err(map_pyerr)
        })?;
        let sol: Solution = Arc::into_inner(py_sol.s)
            .ok_or(LunaModelError::Internal(
                "Solution reference leaked out of backwards scope.".into(),
            ))?
            .into_inner();
        Ok(sol)
    }
}
