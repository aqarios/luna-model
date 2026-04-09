use lunamodel_error::LunaModelError;
use pyo3::PyErr;

pub fn map_pyerr(err: PyErr) -> LunaModelError {
    LunaModelError::WithCause(
        Box::new(LunaModelError::Internal(err.to_string().into())),
        err.into(),
    )
}
