#[cfg(feature = "py")]
use pyo3::exceptions::PyRuntimeError;
#[cfg(feature = "py")]
use pyo3::prelude::*;

#[cfg(feature = "py")]
pub fn overload1<A, V>(py: Python, other: PyObject, fa: fn(&A) -> V) -> PyResult<V>
where
    A: Clone + pyo3::PyClass,
{
    if let Ok(v) = &other.extract::<A>(py) {
        Ok(fa(v))
    } else {
        Err(PyRuntimeError::new_err("unsopported type for operation"))
    }
}

#[cfg(feature = "py")]
pub fn overload2<A, B, V>(
    py: Python,
    other: PyObject,
    fa: fn(&A) -> V,
    fb: fn(&B) -> V,
) -> PyResult<V>
where
    A: Clone + pyo3::PyClass,
    B: Clone + pyo3::PyClass,
{
    if let Ok(v) = &other.extract::<A>(py) {
        Ok(fa(v))
    } else if let Ok(v) = &other.extract::<B>(py) {
        Ok(fb(v))
    } else {
        Err(PyRuntimeError::new_err("unsopported type for operation"))
    }
}

#[cfg(feature = "py")]
pub fn overload3<A, B, C, V>(
    py: Python,
    other: PyObject,
    fa: fn(&A) -> V,
    fb: fn(&B) -> V,
    fc: fn(&C) -> V,
) -> PyResult<V>
where
    A: Clone + pyo3::PyClass,
    B: Clone + pyo3::PyClass,
    C: Clone + pyo3::PyClass,
{
    if let Ok(v) = &other.extract::<A>(py) {
        Ok(fa(v))
    } else if let Ok(v) = &other.extract::<B>(py) {
        Ok(fb(v))
    } else if let Ok(v) = &other.extract::<C>(py) {
        Ok(fc(v))
    } else {
        Err(PyRuntimeError::new_err("unsopported type for operation"))
    }
}
