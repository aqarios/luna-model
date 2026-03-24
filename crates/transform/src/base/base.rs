use std::fmt::Debug;

pub trait BasePass: Debug {
    // + Placeholder {
    fn name(&self) -> String;
    fn requires(&self) -> Vec<String> {
        Vec::new()
    }
    // TODO fn requires_spec(&self) -> ModelSpecs
}
