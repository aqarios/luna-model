use crate::ValueSource;
use derive_more::Deref;

#[repr(transparent)]
/// A wrapper around a [`ValueSource`] that can be converted to and from python with `pyo3`.
#[derive(Deref)]
pub struct PyValueSource(pub ValueSource);

python_enum_wrapper! {
    wrapper: PyValueSource,
    public: ValueSource,
    inner: ValueSource,
    variants: {
        ValueSource::Raw => ("RAW", "Raw"),
        ValueSource::Obj => ("OBJ", "Obj"),
    },
}
