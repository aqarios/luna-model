/// The core component of the library. This includes everyting related to defining and
/// working with an AQ Model (AQ/M) and an AQ Solution (AQ/S).
pub mod core;
/// Collection of all errors that can be returned by this library.
mod errors;
/// Module for importing commonly used structs and functions when working with this library.
pub mod prelude;
/// Everything related to the serialization of all structures that need to be sendable between
/// different workers in a highly efficient manner, both in terms of (de)serialization / (d)e(n)coding.
pub mod serialization;
/// Specific implementaitons for interactions with other modelling libraries and format such as
/// Gurobi, CPLEX and Dimod. This also includes translations of the solution's focused on the
/// translation from another library TO our solution.
pub mod translator;


pub mod transformations;

// Import of the python bindings only when the `--features` flag is set to `py`.
#[cfg(feature = "py")]
mod py_bindings;
