use std::sync::Arc;

use lunamodel_core::{Model, Solution};
use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_transpiler::{PassContext, ReversiblePass};
use pyo3::{
    Py, PyAny, PyErr, Python,
    types::{PyAnyMethods, PyModule, PyTuple, PyTupleMethods},
};

use super::{PyTransformationPass, artifact::PyTransformationPassAdapterArtifact};
use crate::{
    PyModel, PySolution,
    transformv2::{
        adapter::transformation::envelope::BackwardEnvelope, context::PyPassContext,
        utils::map_pyerr,
    },
};

pub struct PyTransformationPassAdapter {
    inner: Py<PyTransformationPass>,
    name: String,
    requires: Vec<String>,
    invalidates: Vec<String>,
}

impl PyTransformationPassAdapter {
    pub fn new(py: Python, inner: Py<PyTransformationPass>) -> LunaModelResult<Self> {
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
        let invalidates: Vec<String> = inner
            .call_method0(py, "invalidates")
            .map_err(map_pyerr)?
            .extract(py)
            .map_err(map_pyerr)?;

        Ok(Self {
            inner,
            name,
            requires,
            invalidates,
        })
    }
}

impl ReversiblePass for PyTransformationPassAdapter {
    type Artifact = PyTransformationPassAdapterArtifact;

    const ID: &'static str = "lunamodel::PyTransformationPassAdapter";

    fn name(&self) -> &str {
        &self.name
    }

    fn forward(&self, model: &mut Model, ctx: &PassContext) -> LunaModelResult<Self::Artifact> {
        let (py_model, py_artifact, backward_envelope) = Python::attach(|py| {
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
            if t.len() != 2 {
                return Err(LunaModelError::Internal(
                    "expected (_model, _artifact)".into(),
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

            Ok((
                py_model,
                py_artifact_obj,
                BackwardEnvelope { module, qualname },
            ))
        })?;
        *model = Arc::into_inner(py_model.0.m)
            .ok_or(LunaModelError::Internal(
                "Model reference leaked out of forward scope.".into(),
            ))?
            .into_inner();
        Ok(PyTransformationPassAdapterArtifact {
            artifact: py_artifact,
            backward: backward_envelope,
        })
    }

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

    fn requires(&self) -> &[String] {
        &self.requires
    }

    fn invalidates(&self) -> &[String] {
        &self.invalidates
    }
}
