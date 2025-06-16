
use crate::transformations::base_passes::Pass;


pub trait PyPass {
    fn as_pass(self) -> Pass;
}
