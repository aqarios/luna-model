use std::fmt::Debug;

// #[cfg(feature = "py")]
// pub trait AsPyPass {
//     type PyPass;
//
//     fn as_pypass(&self) -> Self::PyPass;
// }
//
// #[cfg(feature = "py")]
// pub trait Placeholder: lunamodel_python::transform::AsPyPass {}
//
// #[cfg(not(feature = "py"))]
// pub trait Placeholder {}

pub trait BasePass: Debug {
    // + Placeholder {
    fn name(&self) -> String;
    fn requires(&self) -> Vec<String> {
        Vec::new()
    }
    // TODO fn requires_spec(&self) -> ModelSpecs
}

// impl<T: BasePass> Placeholder for T {}
