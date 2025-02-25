pub mod environment;
pub mod exceptions;
pub mod expression;
mod model;
pub mod operations;
mod term;
mod variable;

pub use environment::Environment;
pub use expression::Expression;
pub use expression::ExpressionBase;
pub use model::Model;
pub use variable::Bounds;
pub use variable::VarId;
pub use variable::VarRef;
pub use variable::Vtype;

// todo: move the python exceptions to the py_bindings module
#[cfg(feature = "py")]
pub use exceptions::MultipleActiveEnvironmentsException;
#[cfg(feature = "py")]
pub use exceptions::NoActiveEnvironmentFoundException;
#[cfg(feature = "py")]
pub use exceptions::VariableExistsException;
#[cfg(feature = "py")]
pub use exceptions::VariablesFromDifferentEnvsException;
