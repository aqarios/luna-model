//! Python wrapper for the model-part enum used by pass invalidation.

// pyo3's `from_py_object` derive emits `.clone()` on these `Copy` types.
#![allow(clippy::clone_on_copy)]

use lunamodel_types::{EnumSet, ModelPart};
use pyo3::{prelude::*, pyclass, types::PySet};

/// Python-facing wrapper for [`ModelPart`].
#[derive(Copy, PartialEq, Hash, Clone, Debug, Eq)]
#[pyclass(from_py_object, eq, eq_int, name = "PyModelPart")]
pub enum PyModelPart {
    Objective,
    Constraints,
    Variables,
    Sense,
}

impl From<ModelPart> for PyModelPart {
    /// Converts the Rust model-part enum into its Python wrapper.
    fn from(value: ModelPart) -> Self {
        match value {
            ModelPart::Objective => PyModelPart::Objective,
            ModelPart::Constraints => PyModelPart::Constraints,
            ModelPart::Variables => PyModelPart::Variables,
            ModelPart::Sense => PyModelPart::Sense,
        }
    }
}

impl From<PyModelPart> for ModelPart {
    /// Converts the Python model-part wrapper back into the core enum.
    fn from(val: PyModelPart) -> Self {
        match val {
            PyModelPart::Objective => ModelPart::Objective,
            PyModelPart::Constraints => ModelPart::Constraints,
            PyModelPart::Variables => ModelPart::Variables,
            PyModelPart::Sense => ModelPart::Sense,
        }
    }
}

/// Converts a model-part set into a Python `set[luna_model.ModelPart]`.
pub fn parts_to_py(py: Python<'_>, parts: EnumSet<ModelPart>) -> PyResult<Bound<'_, PySet>> {
    let model_part = py
        .import("luna_model.transformation.model_part")?
        .getattr("ModelPart")?;
    let members = parts
        .iter()
        .map(|p| model_part.call1((p.to_string(),)))
        .collect::<PyResult<Vec<_>>>()?;
    PySet::new(py, members)
}

/// Reads a Python iterable of `luna_model.ModelPart` (or raw `PyModelPart`) members.
pub fn parts_from_py(obj: &Bound<'_, PyAny>) -> PyResult<EnumSet<ModelPart>> {
    let mut parts = EnumSet::empty();
    for item in obj.try_iter()? {
        let item = item?;
        let part = match item.extract::<PyModelPart>() {
            Ok(part) => part,
            Err(_) => item.getattr("_val")?.extract::<PyModelPart>()?,
        };
        parts.insert(part.into());
    }
    Ok(parts)
}
