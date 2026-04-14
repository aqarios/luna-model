//! # LunaModel: Symbolic modeling for optimization
//!
//! LunaModel is a high-performance symbolic modeling library for describing, translating and
//! transforming optimization problems. It provides the following high-level features:
//!     - System for defining symbolic algebraic expressions of arbitrary degree, constraints and
//!       optimization models (like dimod, gurobi or cplex)
//!     - Translations from and to an LunaModel for many common optimization model formats (like LP)
//!     - Transformations to map an LunaModel from a general model to a specific model, such as transforming a
//!       Constrained (Binary) Quadratic Model (CQM) to a (Unconstrained) Binary Quadratic Model (BQM),
//!       or from an Integer Model to a Binary Model.
//!     - Builtin serialization for maximum portability

pub use lunamodel_core as core;
pub use lunamodel_error as error;
pub use lunamodel_types as types;
pub use lunamodel_utils as utils;

#[cfg(feature = "hashing")]
pub use lunamodel_hashing as hashing;

#[cfg(any(feature = "io", feature = "py-io"))]
pub use lunamodel_io as io;

#[cfg(feature = "python")]
pub use lunamodel_python as python;

#[cfg(feature = "serializer")]
pub use lunamodel_serializer as serializer;

#[cfg(any(feature = "transformv2", feature = "py-transform"))]
pub use lunamodel_transformv2 as transform;

#[cfg(feature = "translate")]
pub use lunamodel_translate as translate;

#[cfg(feature = "unwind")]
pub use lunamodel_unwind as unwind;
