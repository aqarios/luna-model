mod utils;

/// The installed LunaModel version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
/// The core component of the library. This includes everyting related to defining and
/// working with an LunaModel and an LunaModel Solution.
pub mod core;
/// Collection of all errors that can be returned by this library.
pub mod errors;
/// Defines components required to compute the hash of a model.
pub mod hashing;
/// Module for importing commonly used structs and functions when working with this library.
pub mod prelude;
/// Everything related to the serialization of all structures that need to be sendable between
/// different workers in a highly efficient manner, both in terms of (de)serialization / (d)e(n)coding.
pub mod serialization;
/// Specific implementaitons for interactions with other modelling libraries and format such as
/// Gurobi, CPLEX and Dimod. This also includes translations of the solution's focused on the
/// translation from another library TO our solution.
pub mod translator;
/// Common types used in LunaModel.
pub mod types;

#[cfg(feature = "transformations")]
pub mod transformations;

// Import of the python bindings only when the `--features` flag is set to `py`.
#[cfg(feature = "py")]
pub mod py_bindings;

/// Unicode character for printing stuff.
pub mod unicode;
