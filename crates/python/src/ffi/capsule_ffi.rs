use pyo3::{prelude::*, types::PyCapsule};

pub trait CapsuleFFI<'py, R = Bound<'py, PyCapsule>>
where
    Self: Sized,
{
    fn to_capsule(&self, py: Python<'py>) -> PyResult<R>;
    fn from_capsule(capsule: R) -> PyResult<Self>;
}
