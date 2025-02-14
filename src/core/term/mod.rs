mod hashmap;
mod vecs;

pub use hashmap::constant::Constant;
pub use hashmap::higher_order::HigherOrder;
pub use hashmap::linear::Linear;
pub use hashmap::quadratic::Quadratic;
pub use hashmap::quadratic::QuadraticKeyContains;

pub use hashmap::variable_storage::Variables;

// QuadraticModel specialized
// pub use vecs::QuadraticModel;
pub use vecs::QuadraticModelBase;
