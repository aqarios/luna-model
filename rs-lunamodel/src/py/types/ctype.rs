use crate::Ctype;
use derive_more::Deref;

#[repr(transparent)]
/// A wrapper around a [`Ctype`] that can be converted to and from python with `pyo3`.
#[derive(Deref)]
pub struct PyCtype(pub Ctype);

python_enum_wrapper! {
    wrapper: PyCtype,
    public: Ctype,
    inner: Ctype,
    variants: {
        Ctype::Unconstrained => ("UNCONSTRAINED", "Unconstrained"),
        Ctype::Equality => ("EQUALITY", "Equality"),
        Ctype::Inequality => ("INEQUALITY", "Inequality"),
        Ctype::LessEqual => ("LESS_EQUAL", "LessEqual"),
        Ctype::GreaterEqual => ("GREATER_EQUAL", "GreaterEqual"),
    },
}
