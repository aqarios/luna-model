use std::fmt::Debug;

#[cfg(feature = "py")]
use crate::py::IntoAnyPass;

#[cfg(feature = "py")]
pub trait Placeholder: IntoAnyPass {}

#[cfg(not(feature = "py"))]
pub trait Placeholder {}

pub trait BasePass: Debug + Placeholder {
    fn name(&self) -> String;
    fn requires(&self) -> Vec<String> {
        Vec::new()
    }
    // TODO fn requires_spec(&self) -> ModelSpecs
}

impl<T: BasePass> Placeholder for T {}
