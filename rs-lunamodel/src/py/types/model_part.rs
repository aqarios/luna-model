use crate::types::ModelPart;
use derive_more::Deref;

#[repr(transparent)]
/// A wrapper around a [`ModelPart`] that can be converted to and from python with `pyo3`.
#[derive(Deref)]
pub struct PyModelPart(pub ModelPart);

python_enum_wrapper! {
    wrapper: PyModelPart,
    public: ModelPart,
    inner: ModelPart,
    variants: {
        ModelPart::Objective => ("OBJECTIVE", "Objective"),
        ModelPart::Constraints => ("CONSTRAINTS", "Constraints"),
        ModelPart::Variables => ("VARIABLES", "Variables"),
        ModelPart::Sense => ("SENSE", "Sense"),
    },
}
