use derive_more::Deref;
use rs_lunamodel::Sense;

#[repr(transparent)]
/// A wrapper around a [`Sense`] that can be converted to and from python with `pyo3`.
#[derive(Deref)]
pub struct PySense(pub Sense);

python_enum_wrapper! {
    wrapper: PySense,
    public: Sense,
    inner: Sense,
    variants: {
        Sense::Min => ("MIN", "Min"),
        Sense::Max => ("MAX", "Max"),
    },
}
