use std::fmt::Debug;

pub trait Placeholder {}

pub trait BasePass: Debug + Placeholder {
    fn name(&self) -> String;
    fn requires(&self) -> Vec<String> {
        Vec::new()
    }
    // TODO fn requires_spec(&self) -> ModelSpecs
}

impl<T: BasePass> Placeholder for T {}
