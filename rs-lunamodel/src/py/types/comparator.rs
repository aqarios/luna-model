use crate::Comparator;
use derive_more::Deref;

#[repr(transparent)]
/// A wrapper around a [`Comparator`] that can be converted to and from python with `pyo3`.
#[derive(Deref)]
pub struct PyComparator(pub Comparator);

python_enum_wrapper! {
    wrapper: PyComparator,
    public: Comparator,
    inner: Comparator,
    variants: {
        Comparator::Eq => ("EQ", "Eq"),
        Comparator::Le => ("LE", "Le"),
        Comparator::Ge => ("GE", "Ge"),
    },
}
