use lunamodel_error::LunaModelError;
use lunamodel_transpiler::TransformationRecord;
use pyo3::{PyErr, Python, types::PyAnyMethods};

use crate::transform::PyTransformationRecord;

/// Bridge any transpiler error (kind) into a `PyErr`, re-attaching a recovered
/// `TransformationRecord` (when present) as `.record` on the raised exception.
pub fn to_pyerr(err: impl Into<LunaModelError>) -> PyErr {
    let err = err.into();
    // recover() borrows &err; clone the record out before we consume `err`.
    let record = err.recover::<TransformationRecord>().cloned();
    let pyerr: PyErr = err.into(); // error-crate's `From<Lme> for PyErr`
    if let Some(rec) = record {
        Python::attach(|py| {
            let _ = pyerr
                .value(py)
                .setattr("record", PyTransformationRecord::from(rec));
        });
    }
    pyerr
}
