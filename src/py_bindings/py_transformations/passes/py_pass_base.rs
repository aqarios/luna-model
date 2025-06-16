
use crate::transformations::base_passes::ConcretePass;


pub trait PyPass {
    fn as_pass(self) -> ConcretePass;
}
