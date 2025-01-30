mod exceptions;
mod model;
mod utils;

// mod term;
// pub mod expression;
// pub mod varref;
// pub use model::Model;

// new stuff that is cleaner.
mod environment;
mod expression;
mod operations;
mod term;
mod variable;

pub use environment::Environment;
pub use exceptions::VariableExistsException;
pub use variable::Bounds;
pub use variable::VarRef;
pub use variable::Vtype;
