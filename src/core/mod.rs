pub mod environment;
mod exceptions;
mod expression;
// pub mod higher_order_operations;
mod model;
// pub mod operations;
mod term;
mod variable;

pub use environment::Environment;
pub use expression::Expression;
pub use expression::ExpressionBase;
pub use expression::ExpressionBaseInternal;
pub use model::Model;
pub use variable::Bounds;
pub use variable::VarId;
pub use variable::VarRef;
pub use variable::Vtype;

// pub use term::QuadraticExpression;
// pub use term::QuadraticExpressionBase;
// pub use term::QuadraticExpressionBaseInternal;
// pub use term::{Linear, Quadratic};

// todo: move the python exceptions to the py_bindings module
#[cfg(feature = "py")]
pub use exceptions::MultipleActiveEnvironmentsException;
#[cfg(feature = "py")]
pub use exceptions::NoActiveEnvironmentFoundException;
#[cfg(feature = "py")]
pub use exceptions::VariableExistsException;
#[cfg(feature = "py")]
pub use exceptions::VariablesFromDifferentEnvsException;
