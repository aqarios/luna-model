//! Python binding layer for LunaModel.
//!
//! This crate exposes the Rust core to Python through `pyo3`. It is not just a
//! thin FFI shim: it also owns Python-facing type wrappers, argument coercion,
//! pretty-print behavior, transformation adapters, and translator entry points.
mod args;
mod bounds;
mod constraint;
mod environment;
mod expression;
mod model;
mod sol;
mod specs;
mod timer;
pub mod transform;
pub mod translate;
mod types;
mod utilities;
mod utils;
mod variable;

pub mod ffi;

pub mod prelude;

/// Python wrapper for variable bounds.
pub use bounds::{PyBounds, PyBoundsContent, PyUnbounded};
/// Python wrappers for constraints and constraint collections.
pub use constraint::{PyConstraint, PyConstraintCollection, PyConstraintCollectionIterator};
/// Python environment wrapper.
pub use environment::PyEnvironment;
/// Python wrappers for expressions and their internal sparse components.
pub use expression::{
    PyConstant, PyExprContent, PyExpression, PyExpressionIterator, PyHigherOrder, PyLinear,
    PyQuadratic,
};
/// Python model wrapper and associated metadata helpers.
pub use model::{PyModel, PyModelMetadata};
/// Python solution wrapper.
pub use sol::PySolution;
/// Python model specification wrapper.
pub use specs::PyModelSpecs;
/// Python timer/timing wrappers.
pub use timer::{PyTimer, PyTiming};
/// Python variable wrapper.
pub use variable::PyVariable;

/// Re-exported core enum used by Python solution APIs.
pub use lunamodel_core::ValueSource;
/// Re-exported translation target enum used by Python APIs.
pub use lunamodel_translate::TranslationTarget;
/// Re-exported core enums commonly surfaced in Python.
pub use lunamodel_types::{Comparator, Ctype, Sense, Vtype};

/// Python enum wrappers corresponding to the re-exported core enums.
pub use types::{PyComparator, PyCtype, PySense, PyTranslationTarget, PyValueSource, PyVtype};

/// Python exception types generated from the Rust error taxonomy.
pub use lunamodel_error::py::*;
/// Python-facing quicksum helper.
pub use utilities::quicksum;
