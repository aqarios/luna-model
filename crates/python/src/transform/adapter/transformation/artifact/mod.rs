//! Artifact wrapper types for Python-defined transformation passes.

use lunamodel_error::LunaModelResult;
use lunamodel_transpiler::Artifact;
use pyo3::{
    Py, PyAny, PyErr, Python, pyclass,
    types::{PyAnyMethods, PyBytes, PyBytesMethods, PyModule},
};

use crate::transform::{
    adapter::transformation::envelope::BackwardEnvelope, envelope::Envelope, utils::map_pyerr,
};

mod envelope;

use envelope::ArtifactEnvelope;

#[pyclass]
pub struct PyTransformationPassAdapterArtifact {
    pub artifact: Py<PyAny>,
    pub backward: BackwardEnvelope,
}

impl Artifact for PyTransformationPassAdapterArtifact {
    fn static_type_tag() -> &'static str
    where
        Self: Sized,
    {
        "lunamodel::PyTransformationPassAdapterArtifact"
    }

    fn serialize(&self) -> LunaModelResult<Vec<u8>> {
        Python::attach(|py| {
            let obj = self.artifact.bind(py);
            let cls = obj.getattr("__class__").map_err(map_pyerr)?;

            let module: String = cls
                .getattr("__module__")
                .map_err(map_pyerr)?
                .extract()
                .map_err(map_pyerr)?;

            let qualname: String = cls
                .getattr("__qualname__")
                .map_err(map_pyerr)?
                .extract()
                .map_err(map_pyerr)?;

            let content = obj
                .call_method0("serialize")
                .map_err(map_pyerr)?
                .extract::<Py<PyBytes>>()
                .map_err(PyErr::from)
                .map_err(map_pyerr)?
                .bind(py)
                .as_bytes()
                .to_vec();

            Ok(ArtifactEnvelope {
                module,
                qualname,
                content,
                backward: self.backward.clone(),
            }
            .encode())
        })
    }

    fn deserialize(bytes: &[u8]) -> LunaModelResult<Self>
    where
        Self: Sized,
    {
        let enve = ArtifactEnvelope::decode(bytes)?;
        Python::attach(|py| {
            let module = PyModule::import(py, &enve.module).map_err(map_pyerr)?;
            let cls = module.getattr(&enve.qualname).map_err(map_pyerr)?;
            let payload = PyBytes::new(py, &enve.content);
            let obj = cls
                .call_method1("deserialize", (payload,))
                .map_err(map_pyerr)?;
            Ok(Self {
                artifact: obj.unbind(),
                backward: enve.backward,
            })
        })
    }
}
