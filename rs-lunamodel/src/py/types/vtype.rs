use derive_more::Deref;
use crate::Vtype;

#[repr(transparent)]
/// A wrapper around a [`Vtype`] that can be converted to and from python with `pyo3`.
#[derive(Deref)]
pub struct PyVtype(pub Vtype);

python_enum_wrapper! {
    wrapper: PyVtype,
    public: Vtype,
    inner: Vtype,
    variants: {
        Vtype::Binary => ("BINARY", "Binary"),
        Vtype::InvertedBinary => ("INVERTED_BINARY", "InvertedBinary"),
        Vtype::Spin => ("SPIN", "Spin"),
        Vtype::Integer => ("INTEGER", "Integer"),
        Vtype::Real => ("REAL", "Real"),
    },
}
